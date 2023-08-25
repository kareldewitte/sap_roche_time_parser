import Plotly from 'plotly.js-dist-min'
import './css/input.css';



import('./pkg')
  .then(rust_module => {
    
    let parser = null;
    console.log("setting up");
    if (parser === null) {
      parser = rust_module;
    }
    const reader = new FileReader();
    const loader = document.getElementById("file_submit");
    
    loader.addEventListener("input", event=>{
      console.log("loaded");
      reader.readAsArrayBuffer(loader.files[0]);
    })
    
  
    reader.onload = function(ev) {  
      console.log("passing " +reader.result.byteLength);

      console.log(parser.read_file(reader.result.byteLength,new Uint8Array(reader.result)));
    }

  })
  .catch(console.error);