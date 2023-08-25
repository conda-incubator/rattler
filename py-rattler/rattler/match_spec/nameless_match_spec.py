from __future__ import annotations
from typing import Self, TYPE_CHECKING

from rattler.rattler import PyNamelessMatchSpec

if TYPE_CHECKING:
    from rattler.match_spec import MatchSpec
    from rattler.repo_data import PackageRecord


class NamelessMatchSpec:
    """
    Similar to a `MatchSpec` but does not include the package name.
    This is useful in places where the package name is already known
    (e.g. `foo = "3.4.1 *cuda"`).
    """

    def __init__(self, spec: str):
        if isinstance(spec, str):
            self._nameless_match_spec = PyNamelessMatchSpec(spec)
        else:
            raise TypeError(
                "NamelessMatchSpec constructor received unsupported type"
                f" {type(spec).__name__!r} for the 'spec' parameter"
            )

    def matches(self, package_record: PackageRecord) -> bool:
        """
        Match a MatchSpec against a PackageRecord
        """
        return self._nameless_match_spec.matches(package_record._package_record)

    @classmethod
    def _from_py_nameless_match_spec(
        cls, py_nameless_match_spec: PyNamelessMatchSpec
    ) -> Self:
        """
        Construct py-rattler NamelessMatchSpec from PyNamelessMatchSpec FFI object.
        """
        nameless_match_spec = cls.__new__(cls)
        nameless_match_spec._nameless_match_spec = py_nameless_match_spec

        return nameless_match_spec

    @classmethod
    def from_match_spec(cls, spec: MatchSpec) -> Self:
        """
        Constructs a NamelessMatchSpec from a MatchSpec.
        """
        return cls._from_py_nameless_match_spec(
            PyNamelessMatchSpec.from_match_spec(spec._match_spec)
        )

    def __str__(self) -> str:
        return self._nameless_match_spec.as_str()

    def __repr__(self) -> str:
        return self.__str__()
