from rich.console import Console
from rich.panel import Panel
from rich.live import Live
from rich.spinner import Spinner
from rich.table import Table
from mockai._core import main

__all__ = ["serve"]

def serve():
    console = Console()
    
    # Show welcome banner
    welcome_panel = Panel.fit(
        "[bold green]Mock AI Server[/bold green]\n[cyan]Your Local AI Development Environment[/cyan]",
        border_style="blue",
        title="Welcome",
        subtitle="v1.0.0"
    )
    console.print(welcome_panel)
    
    # Show server info
    table = Table(show_header=True, header_style="bold magenta")
    table.add_column("Setting", style="cyan")
    table.add_column("Value", style="green")
    table.add_row("Host", "localhost")
    table.add_row("Port", "8000")
    table.add_row("Mode", "Development")
    console.print(table)
    
    try:
        # Show loading spinner while starting
        with Live(console=console) as live:
            spinner = Spinner('dots', text="Starting server...")
            live.update(spinner)
            main()
    except KeyboardInterrupt:
        console.print("\n[bold red]⚠[/bold red] [cyan]Mock AI server shutting down...[/cyan]")
        console.print("[dim]Thanks for using Mock AI![/dim] 👋\n")
        console.print(Panel.fit(
            "[yellow]Server stopped safely[/yellow]",
            border_style="red",
            title="Shutdown Complete"
        ))
