schema Person:
    name: str = "kcl"
    age: int = 1
    bla?: float

x0 = Person{bla: 0.4}
x1 = Person{age=101}
