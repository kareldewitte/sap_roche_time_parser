// import index from './dist/index.html'
// //import indexjs from './dist/index.js'

// addEventListener('fetch', event => {
//     event.respondWith(handleRequest(event.request));
//   })
  
//   async function handleRequest(request) {

//     const { pathname } = new URL(request.url)
//     console.log(pathname);

//     if (pathname === '/') {
//         return new Response(index, {
//         headers: { 'content-type': 'text/html' },
//         })
//     }else{
        
//         const response = await fetch(request);    
//         return response
//     }
    

    

//   }

import { getAssetFromKV } from '@cloudflare/kv-asset-handler';
import manifestJSON from '__STATIC_CONTENT_MANIFEST';
const assetManifest = JSON.parse(manifestJSON);

export default {
  async fetch(request, env, ctx) {
    try {
      // Add logic to decide whether to serve an asset or run your original Worker code
      return await getAssetFromKV(
        {
          request,
          waitUntil: ctx.waitUntil.bind(ctx),
        },
        {
          ASSET_NAMESPACE: env.__STATIC_CONTENT,
          ASSET_MANIFEST: assetManifest,
        }
      );
    } catch (e) {
      let pathname = new URL(request.url).pathname;
      return new Response(`"${pathname}" not found`, {
        status: 404,
        statusText: 'not found',
      });
    }
  },
};