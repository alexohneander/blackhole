import { useState } from 'react';
import { open } from '@tauri-apps/api/dialog';
import { invoke } from "@tauri-apps/api/tauri";

function UploadFile() {
    const [uploadMsg, setUploadMsg] = useState("");

    async function selectFile() {
        const selected = await open({
            multiple: true,
        });


        if (Array.isArray(selected) && selected.length > 1) {
            // user selected multiple files
            // TODO: Zip the files, encrypt the Zip and send them to the server
            console.log("Test: ", selected);
        } else if (selected === null) {
            // user cancelled the selection
        } else {
            // user selected a single file
            // TODO: Encrypt the file and send it to the server
            console.log("Client Side File: ", selected);
            const file = selected[0];

            setUploadMsg(await invoke("upload_file", { path: file }));
        }
    }

    return (
        <div className="upload-file">
            <input type="text" className="form-control" value={uploadMsg} readOnly /> <br />
            <button type="button" className="btn btn-primary" onClick={selectFile}>Upload File</button>
        </div>
    );
}

export default UploadFile;