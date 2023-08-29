import Plotly from 'plotly.js-dist-min'
import './css/input.css';
import './webcomponents/list-card.js';


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
      let json_res = JSON.parse(parser.read_file(reader.result.byteLength,new Uint8Array(reader.result)));
      // let comp = document.getElementById("presenter");
      // comp.data = json_res;
      
      //console.log(json_res);

      var data = [{
        values: [json_res["time_travel"], json_res["time_home"], json_res["time_office"]],
        labels: ['Travel', 'Home', 'Onsite'],
        type: 'pie'
      }];
      //console.log(json_res.times);
      
      var layout = {
        height: 400,
        width: 500
      };
            
      Plotly.newPlot('diagram', data, layout);
      const details = document.querySelector("#rows");
      
      json_res.times.reverse().forEach(x => {
        console.log(x);
        details.insertAdjacentHTML(
          'afterend',
          "<div class='table-row text-xs text-gray-700 text-center'>"+
          "<div class='table-cell'>"+x.day+"</div>"+
          "<div class='table-cell'>"+x.timeType+"</div>"+
          "<div class='table-cell'>"+x.time+"</div>"

          +"</div>",
        );

        });
      
      

    
    
    }






  })
  .catch(console.error);