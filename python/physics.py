from ezvectors import Vector2


def point_polygon(x, y, polygon):
    inside = False
    j = len(polygon) - 1
    for i in range(len(polygon)):
        xi, yi = polygon[i]
        xj, yj = polygon[j]

        intersects = (yi > y) != (yj > y)

        if intersects:
            intersection_x = ((xj - xi) * (y - yi) / (yj - yi)) + xi

            if intersection_x > x:
                inside = not inside

        j = i
    return inside


def point_side(a, b, p):
    a = Vector2(a)
    b = Vector2(b)
    p = Vector2(p)
    ab = b - a
    ap = p - a
    return ab.cross(ap)


def segment_intersect(a, b, c, d):
    ab_c = point_side(a, b, c)
    ab_d = point_side(a, b, d)

    cd_a = point_side(c, d, a)
    cd_b = point_side(c, d, b)

    return ((ab_c > 0) != (ab_d > 0)) and ((cd_a > 0) != (cd_b > 0))


def polygon_aabb(polygon):
    xs = []
    ys = []

    for x, y in polygon:
        xs.append(x)
        ys.append(y)

    return (min(xs), max(xs), min(ys), max(ys))


def aabb_overlap(a, b):
    a_min_x, a_max_x, a_min_y, a_max_y = a
    b_min_x, b_max_x, b_min_y, b_max_y = b

    return (
        a_min_x < b_max_x
        and a_max_x > b_min_x
        and a_min_y < b_max_y
        and a_max_y > b_min_y
    )


def polygon_concave(poly1, poly2):

    aabb1 = polygon_aabb(poly1)
    aabb2 = polygon_aabb(poly2)

    if not aabb_overlap(aabb1, aabb2):
        return False

    for x, y in poly1:
        if point_polygon(x, y, poly2):
            return True
    for x, y in poly2:
        if point_polygon(x, y, poly1):
            return True
    for i in range(len(poly1)):
        a = poly1[i]
        b = poly1[i - 1]
        for j in range(len(poly2)):
            c = poly2[j]
            d = poly2[j - 1]
            if segment_intersect(a, b, c, d):
                return True
    return False


def project_polygon(axis, polygon):
    projections = [Vector2(point).dot(axis) for point in polygon]

    return min(projections), max(projections)


def overlap(min_a, max_a, min_b, max_b):
    return max_a >= min_b and max_b >= min_a


def polygon_axes(polygon):
    axes = []

    for i in range(len(polygon)):
        p1 = Vector2(polygon[i])
        p2 = Vector2(polygon[i - 1])

        edge = p2 - p1
        axis = Vector2(-edge.y, edge.x).normalize()

        axes.append(axis)
    return axes


def get_overlap(min_a, max_a, min_b, max_b):
    return min(max_a, max_b) - max(min_a, min_b)


def polygon_collision(poly1, poly2):
    aabb1 = polygon_aabb(poly1)
    aabb2 = polygon_aabb(poly2)

    if not aabb_overlap(aabb1, aabb2):
        return False

    axes = polygon_axes(poly1) + polygon_axes(poly2)

    for axis in axes:
        min_a, max_a = project_polygon(axis, poly1)
        min_b, max_b = project_polygon(axis, poly2)

        if not overlap(min_a, max_a, min_b, max_b):
            return False

    return True


def polygon_center(polygon):
    total = Vector2(0, 0)

    for point in polygon:
        total += Vector2(point)

    return total / len(polygon)


def advanced_collision(poly1, poly2):
    smallest_overlap = float("inf")
    smallest_axis = None
    aabb1 = polygon_aabb(poly1)
    aabb2 = polygon_aabb(poly2)

    if not aabb_overlap(aabb1, aabb2):
        return (False, None, 0)

    axes = polygon_axes(poly1) + polygon_axes(poly2)

    for axis in axes:
        min_a, max_a = project_polygon(axis, poly1)
        min_b, max_b = project_polygon(axis, poly2)

        if not overlap(min_a, max_a, min_b, max_b):
            return (False, None, 0)

        overlap_amount = get_overlap(min_a, max_a, min_b, max_b)

        if overlap_amount < smallest_overlap:
            smallest_overlap = overlap_amount
            smallest_axis = axis

    center1 = polygon_center(poly1)
    center2 = polygon_center(poly2)
    direction = center2 - center1

    if direction.dot(smallest_axis) < 0:
        smallest_axis = -smallest_axis

    return (True, smallest_axis, smallest_overlap)
