fn main() {
        
        windows::build!(    
            
            Windows::Storage::Pickers::{
                FileOpenPicker,PickerLocationId,PickerViewMode,
            },

            Windows::Storage::{
                StorageFile,StorageFolder,
            },
            Windows::Foundation::Collections::IVector,
            Windows::Foundation::{
                TimeSpan,
                IAsyncOperation,
            },
            Windows::Media::Render::AudioRenderCategory,
            Windows::Media::Audio::{
                AudioGraph,AudioGraphSettings,CreateAudioGraphResult,CreateAudioDeviceOutputNodeResult,AudioDeviceOutputNode,CreateAudioFileInputNodeResult,AudioFileInputNode,
            },
        );
    }