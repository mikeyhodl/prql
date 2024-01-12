from typing import List, Optional

class CompileOptions:
    def __init__(
        self,
        *,
        format: bool = True,
        target: str = "sql.any",
        signature_comment: bool = True,
    ) -> None: ...

def compile(prql_query: str, options: Optional[CompileOptions] = None) -> str: ...
def prql_to_pl(prql_query: str) -> str: ...
def pl_to_rq(pl_json: str) -> str: ...
def rq_to_sql(rq_json: str) -> str: ...
def get_targets() -> List[str]: ...

__version__: str
