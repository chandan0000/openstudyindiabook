use serde_json::Value;
use socketioxide::{
    extract::{AckSender, Bin, Data, SocketRef},
};
use serde::{Deserialize, Serialize};
use tracing::info;


#[derive(Deserialize,Serialize, Debug)]
struct Message {
    
    txt:String
}

pub fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    info!("haders : {:?} {:?}", socket.req_parts().headers.get("data"), socket.req_parts().uri.query());
    socket.req_parts().uri.query();
    info!(
        "Socket.IO connected: {:?} {:?} + data {:?}",
        socket.ns(),
        socket.id,
        data
    );
    socket.emit("auth", data).ok();

    socket.on(
        "message",
        |socket: SocketRef, Data::<Message>(data), Bin(bin)| {
            // let a=data.get("txt").unwrap();
            info!("Received event: {:?} {:?}", data.txt, bin);
            socket.emit("message", data);

            // socket.bin(bin).emit("message-back", data.txt).ok();
        },
    );

    socket.on(
        "message-with-ack",
        |Data::<Value>(data), ack: AckSender, Bin(bin)| {
            info!("Received event: {:?} {:?}", data, bin);
            ack.bin(bin).send(data).ok();
        },
    );
}
