use tokio::runtime::Runtime;
use tonbo::{executor::tokio::TokioExecutor, tonbo_record, DB};
use bevy::{app::App, prelude::{Plugin, Resource}};

#[tonbo_record]
pub struct Node {
    #[primary_key]
    pub(crate) id: u64,
    pub(crate) name: String,
    pub offset_x: i32,
    pub offset_y: i32
}
#[allow(dead_code)]
#[derive(Resource)]
pub struct LocalStorage(DB<Node,TokioExecutor>);

pub struct StoragePlugin;

impl Plugin for StoragePlugin { 
    fn build(&self, app: &mut App) {
        let rt = Runtime::new().unwrap();
        let db: DB<Node, TokioExecutor> = rt.block_on(async{
            let db = DB::new("./db_path/users".into(), TokioExecutor::default()).await.unwrap();
            // db.insert(Node {
            //     id: 123123u64,
            //     name: "Alice".into(),
            //     offset_x: 200,
            //     offset_y: -120,
            // })
            // .await
            // .unwrap();
            db
        });
        app.insert_resource(LocalStorage(db));
    }
}