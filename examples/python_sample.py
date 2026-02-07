from dataclasses import dataclass
from typing import Optional


@dataclass
class User:
    """Represents a user in the system."""

    name: str
    age: int = 0

    def greet(self) -> str:
        """Return a friendly greeting."""
        return f"Hello, {self.name}"

    @property
    def is_adult(self) -> bool:
        """True when age is 18 or higher."""
        return self.age >= 18

    @staticmethod
    def normalize_name(name: str) -> str:
        return name.strip()

    @classmethod
    def from_name(cls, name: str) -> "User":
        return cls(name=name)

    def __repr__(self) -> str:
        return f"User(name={self.name!r})"


class Admin(User):
    """Administrator user."""

    pass


def helper(value: int) -> Optional[str]:
    """Convert positive values to strings."""
    if value > 0:
        return str(value)
    return None
