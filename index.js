import Plotly from 'plotly.js-dist-min'
import './css/input.css';
import './webcomponents/list-card.js';

let times_global={};


function merge(old,nw){
  old.OFF += nw.OFF;
  old.ONSITE += nw.ONSITE;
  old.WFH += nw.WFH;
  old.TRAVEL += nw.TRAVEL;
  old.REM += nw.REM;
  return old;
}

function reorg(objectArray) {
  return objectArray.reduce(
    function (accumulator, currentObject,index,array) {
      
      let prev = array[index-1];
      
      if(prev && currentObject && prev.day==currentObject.day){
        delete array[index];
      }

      if(currentObject && currentObject["day"]=="cont'd"){
          
          if(prev){
            array[index-1]=merge(prev,currentObject);
            delete array[index];
            
          
          }
          return array;
      }

      if(currentObject==undefined){
        delete array[index];
      }
      
      return array;
      }, {});
}

function remap(objectArray) {
  return objectArray.map(
    function (x) {
      let obj = {'day':x.day,'WFH':0,'ONSITE':0,'REM':0,'TRAVEL':0,'OFF':0}
      if(x.timeType!="NA"){
        obj[x.timeType]=x.time;
        return obj;
      }
    }
  )
}



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
     
      let times = reorg(remap(json_res.times));
      console.log(times);
      let wfh_times = [],onsite_times=[],travel_times=[];
      let old_xd = "";
      times.reverse().forEach(x => {
                if(x && x.day!='na'){
                  
                  let onsite = Math.round((x.ONSITE+x.REM+Number.EPSILON)*100)/100;
                  details.insertAdjacentHTML(
                    'afterend',
                    "<div class='table-row text-xs text-gray-700 text-center'>"+
                    "<div class='table-cell'>"+x.day+"</div>"+
                    "<div class='table-cell'>"+x.WFH+"</div>"+
                    "<div class='table-cell'>"+onsite+"</div>"+
                    "<div class='table-cell'>"+x.TRAVEL+"</div></div>",
                  );
                  wfh_times.push(x.WFH);
                  onsite_times.push(onsite);   
                  travel_times.push(x.TRAVEL);  
                  old_xd = x.day;  
                }

        });
       
        sessionStorage.setItem("wfh_times",wfh_times.reverse().join("\n"));
        sessionStorage.setItem("onsite_times",onsite_times.reverse().join("\n"));
        sessionStorage.setItem("travel_times",travel_times.reverse().join("\n"));
    
    }

    




  })
  .catch(console.error);