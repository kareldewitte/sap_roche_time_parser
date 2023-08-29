import {LitElement, html} from 'lit';
import '../css/input.css';


export class ListCard extends LitElement {
    
    static properties = {
        data: {}
    };

    createRenderRoot(){
        return this;
    }
  
    render() {
        
    console.log(this.data);

    return html`
    <div id="results" class="items-center flex mt-3">
    <div class="bg-white max-w-sm mx-auto rounded overflow-hidden shadow-lg">
        <div class="px-6 py-4">
            <div class="font-bold text-xl mb-2">Time consumed</div>
            <p class="text-gray-700 text-base">
            ${['✨', '🔥', '❤️']}
            </p>
        </div>
        <div class="px-6 pt-4 pb-2">
            <span class="inline-block bg-gray-200 rounded-full px-3 py-1 text-sm font-semibold text-gray-700 mr-2 mb-2">#photography</span><br/>
            <span class="inline-block bg-gray-200 rounded-full px-3 py-1 text-sm font-semibold text-gray-700 mr-2 mb-2">#travel</span>
            <span class="inline-block bg-gray-200 rounded-full px-3 py-1 text-sm font-semibold text-gray-700 mr-2 mb-2">
            ${(function* () {
                for (let i = 1; i < 4; i++) yield i;
            })()}
            
            #winter</span>
            </div>
        </div>
    </div>
    </div>
    `
  }
}
customElements.define('list-card', ListCard);