
    use windows::{
        core::Result,
        Storage::{
            StorageFile,
        },
        Media::{
            Audio::{
                AudioGraph,AudioGraphSettings,
            },
            Render::{
                AudioRenderCategory,
            }
        }
    };

use std::io;
use std::{thread, time};

fn main() -> Result<()> {
    
    //Audio Graph作成
    let settings = AudioGraphSettings::Create(AudioRenderCategory::Media)?;
    let result = AudioGraph::CreateAsync(settings)?;
    let audioGraph = result.get()?.Graph()?;



    //wavファイル入力ノード作成
    println!("音声ファイルの絶対パスを入力してください：");
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line"); //file path読み込み
    let pass = line.trim();
    let file = StorageFile::GetFileFromPathAsync(pass)?.get()?;
    let result = audioGraph.CreateFileInputNodeAsync(file)?;
    let fileInputNode = result.get()?.FileInputNode()?;


    //スピーカ出力ノード作成
    let result = audioGraph.CreateDeviceOutputNodeAsync()?;
    let deviceOutputNode = result.get()?.DeviceOutputNode()?;
    
    //入力ノードと出力ノードを接続
    fileInputNode.AddOutgoingConnection(deviceOutputNode);
    


    //開始
    audioGraph.Start()?;
    let play_time = fileInputNode.Duration()?;
    let time2= play_time.Duration * 100; //100ms単位で取得できるので100かける
    thread::sleep(time::Duration::from_nanos(time2 as u64));

    Ok(())
}
