use serde::Serialize;
use swayipc::{Connection, EventType, Fallible};

fn main() -> Fallible<()> {
    let subs = [
        EventType::Workspace,
        EventType::Shutdown,
        EventType::Mode,
        EventType::Output,
        ];

    for _event in Connection::new()?.subscribe(subs)? {
       let workspaces:Box<[Workspace]> = Connection::new()?.get_workspaces()?.iter().map(|workspace|{
            
           Workspace{
                name: Box::from(workspace.name.as_ref()),
                output: Box::from(workspace.output.as_ref()),
                visible: workspace.visible,
                focused: workspace.focused,
                urgent: workspace.urgent,
            }}
        )
        .collect();
        println!("{}", serde_json::to_string(&workspaces).expect("Failed to serialize workspaces"));
    }
    Ok(())
}

#[derive(Serialize)]
struct Workspace{
    name:Box<str>,
    output:Box<str>,
    visible:bool,
    urgent:bool,
    focused:bool,
}