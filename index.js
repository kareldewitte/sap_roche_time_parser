import Plotly from 'plotly.js-dist-min'




import('./pkg')
  .then(rust_module => {
    
    let parser = null;
    console.log("setting up");
    if (parser === null) {
      parser = rust_module;
    }

    const importer = document.getElementById("file_submit");
    importer.addEventListener("input", event => {
      
      console.log("loaded");
      let size = parser.read_file(importer.files[0]);
      console.log(size);
    });


  })
  .catch(console.error);