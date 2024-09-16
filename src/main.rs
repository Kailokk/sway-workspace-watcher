use core::panic;

use serde::Serialize;
use swayipc::{Connection, EventType};

#[derive(Serialize)]
struct Workspace {
    name: Box<str>,
    output: Box<str>,
    visible: bool,
    urgent: bool,
    focused: bool,
}

fn write_workspaces(connection:&mut Connection){
    let workspaces: Box<[Workspace]> = connection
            .get_workspaces()
            .expect("Failed to retrieve workspaces")
            .iter()
            .map(|workspace| Workspace {
                name: Box::from(workspace.name.as_ref()),
                output: Box::from(workspace.output.as_ref()),
                visible: workspace.visible,
                focused: workspace.focused,
                urgent: workspace.urgent,
            })
            .collect();
        println!(
            "{{\"workspaces\":{}}}",
            serde_json::to_string(&workspaces).expect("Failed to serialize workspaces")
        );
}

fn main() {
    let  event_connection = match Connection::new() {
        Ok(conn) => conn,
        Err(error) => match error {
            swayipc::Error::SocketNotFound => {
                panic!("Could not find sway socket, is sway running?")
            }
            _ => panic!("Internal sway error"),
        },
    };

    let mut workspace_connection = match Connection::new() {
        Ok(conn) => conn,
        Err(error) => match error {
            swayipc::Error::SocketNotFound => {
                panic!("Could not find sway socket, is sway running?")
            }
            _ => panic!("Internal sway error"),
        },
    };

    write_workspaces(&mut workspace_connection);

    let Ok(sway_event_stream) = event_connection.subscribe(&[
        EventType::Workspace,
        EventType::Shutdown,
        EventType::Mode,
        EventType::Output,
    ]) else {
        panic!("Something went wrong while subscibing to sway events");
    };

    sway_event_stream.for_each(|_|{
        write_workspaces(&mut workspace_connection);
    });
}
