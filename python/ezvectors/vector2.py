import math

class Vector2:

    __slots__ = ("x", "y")
    
    def __init__(self, x, y=None):

      if y is None:
        x, y = x

      self.x = x
      self.y = y
      
    def __repr__(self):
      return f"Vector2({self.x}, {self.y})"
      
    def __add__(self, other):

      other = Vector2(other)

      return Vector2(
        self.x + other.x,
        self.y + other.y
      )
      
    def __sub__(self, other):

      other = Vector2(other)

      return Vector2(
        self.x - other.x,
        self.y - other.y
      )
      
    def __mul__(self, scalar):
      return Vector2(
        self.x * scalar,
        self.y * scalar
      )
      
    __rmul__ = __mul__
  
    def __truediv__(self, scalar):
      return Vector2(
        self.x / scalar,
        self.y / scalar
      )

    def __neg__(self):
      return Vector2(
        -self.x,
        -self.y
      )
      
    def __iter__(self):
      yield self.x
      yield self.y

    def __getitem__(self, index):
      if index == 0:
          return self.x
  
      if index == 1:
          return self.y
  
      raise IndexError
      
    def magnitude(self):
      return math.sqrt(
        self.x**2 + self.y**2
      )
      
    def magnitude_squared(self):
      return self.x**2 + self.y**2
      
    def normalize(self):
      length = self.magnitude()

      if length == 0:
        return Vector2(0,0)

      return Vector2(
        self.x/length,
        self.y/length
      )
      
    def dot(self, other):

      other = Vector2(other)

      return(
        self.x * other.x + self.y * other.y
      )
      
    def distance_to(self, other):

      other = Vector2(other)

      return(
        (other-self).magnitude()
      )
      
    def to_tuple(self):
      return(self.x, self.y)
      
    @classmethod
    def from_tuple(cls, tup):
      return cls(tup)
      
    def cross(self, other):

      other = Vector2(other)

      return self.x*other.y-self.y*other.x

    def rotate(self, angle):
      return Vector2(
        self.x * math.cos(angle) - self.y * math.sin(angle),
        self.x * math.sin(angle) + self.y * math.cos(angle)
      )
      
    def rotate_deg(self, angle):
      degree = math.radians(angle)
      return self.rotate(degree)

    def angle(self):
      return math.atan2(self.y, self.x)
    