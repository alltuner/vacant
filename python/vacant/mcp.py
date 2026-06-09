"""Model Context Protocol server exposing vacant's domain checks over stdio.

Run it with `vacant mcp` (needs the `mcp` extra: `uvx --from 'vacant[mcp]' vacant mcp`).
"""

from mcp.server.fastmcp import FastMCP
from pydantic import BaseModel

from vacant import check_many

_MAX_DOMAINS = 500

server = FastMCP("vacant")


class DomainStatus(BaseModel):
    domain: str
    status: str


@server.tool()
def check_domains(domains: list[str], verify: bool = False) -> list[DomainStatus]:
    """Check domain availability across any TLD via authoritative DNS.

    Pass one name or many; returns one {domain, status} per input, in order.
    status is one of: registered, available, unconfirmed, reserved, invalid, unknown.

    'available' only ever appears when verify=true, which confirms undelegated
    names against the registry (RDAP). Without it, a name with no DNS delegation
    reports 'unconfirmed' (probably free, but a held or expired domain looks the
    same to DNS). Set verify=true when the user asks whether a specific name is
    free; leave it false to cheaply screen a large list down to candidates.
    """
    cleaned = [d.strip() for d in domains if d.strip() and not d.strip().startswith("#")]
    results = check_many(cleaned[:_MAX_DOMAINS], verify=verify)
    return [DomainStatus(domain=r.domain or r.input, status=r.status.value) for r in results]


def serve() -> None:
    server.run()
