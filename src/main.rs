use swayipc::{Connection, EventType, Fallible};

fn main() -> Fallible<()> {
    let subs = [
        EventType::Workspace,
        EventType::Shutdown,
        EventType::Mode,
        EventType::Output,
        ];
    for _event in Connection::new()?.subscribe(subs)? {
        Connection::new()?.get_workspaces()?.iter().for_each(|workspace|{
            println!("Workspace found \'{}\', with id \'{}\', on output \'{}\', is visible: {}",workspace.name,workspace.id,workspace.output,workspace.visible);
        });
        println!("----------------------------------------------------------------------")
    }
    Ok(())
}