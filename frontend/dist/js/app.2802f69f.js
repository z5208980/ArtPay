(function(e){function t(t){for(var n,o,s=t[0],l=t[1],i=t[2],d=0,b=[];d<s.length;d++)o=s[d],Object.prototype.hasOwnProperty.call(c,o)&&c[o]&&b.push(c[o][0]),c[o]=0;for(n in l)Object.prototype.hasOwnProperty.call(l,n)&&(e[n]=l[n]);u&&u(t);while(b.length)b.shift()();return a.push.apply(a,i||[]),r()}function r(){for(var e,t=0;t<a.length;t++){for(var r=a[t],n=!0,s=1;s<r.length;s++){var l=r[s];0!==c[l]&&(n=!1)}n&&(a.splice(t--,1),e=o(o.s=r[0]))}return e}var n={},c={app:0},a=[];function o(t){if(n[t])return n[t].exports;var r=n[t]={i:t,l:!1,exports:{}};return e[t].call(r.exports,r,r.exports,o),r.l=!0,r.exports}o.m=e,o.c=n,o.d=function(e,t,r){o.o(e,t)||Object.defineProperty(e,t,{enumerable:!0,get:r})},o.r=function(e){"undefined"!==typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},o.t=function(e,t){if(1&t&&(e=o(e)),8&t)return e;if(4&t&&"object"===typeof e&&e&&e.__esModule)return e;var r=Object.create(null);if(o.r(r),Object.defineProperty(r,"default",{enumerable:!0,value:e}),2&t&&"string"!=typeof e)for(var n in e)o.d(r,n,function(t){return e[t]}.bind(null,n));return r},o.n=function(e){var t=e&&e.__esModule?function(){return e["default"]}:function(){return e};return o.d(t,"a",t),t},o.o=function(e,t){return Object.prototype.hasOwnProperty.call(e,t)},o.p="/Artpay=Playground/";var s=window["webpackJsonp"]=window["webpackJsonp"]||[],l=s.push.bind(s);s.push=t,s=s.slice();for(var i=0;i<s.length;i++)t(s[i]);var u=l;a.push([0,"chunk-vendors"]),r()})({0:function(e,t,r){e.exports=r("56d7")},"0b7f":function(e,t,r){"use strict";r("fcea")},1:function(e,t){},2:function(e,t){},"56d7":function(e,t,r){"use strict";r.r(t);r("e260"),r("e6cf"),r("cca6"),r("a79d");var n=r("7a23");function c(e,t,r,c,a,o){var s=Object(n["n"])("HelloWorld");return Object(n["k"])(),Object(n["c"])(s)}var a=r("1da1"),o=(r("96cf"),r("a4d3"),r("e01a"),{class:"my-5"}),s={class:"font-bold text-center"},l={class:"container mx-auto px-16"},i={class:"container"},u={class:"grid grid-cols-5 gap-3 mx-16"},d={class:"col-span-2"},b=Object(n["e"])("h1",{class:"font-bold text-center"},"ArtPay NFT",-1),w=Object(n["g"])(" Login to continue! "),p=Object(n["e"])("h2",{class:"font-bold"},"Mint new NFT",-1),g={class:"mb-6"},m=Object(n["e"])("label",{class:"block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300"},"Token Id",-1),f={class:"mb-6"},j=Object(n["e"])("label",{class:"block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300"},"Licence / Copyright agreement",-1),v={class:"mb-6"},O=Object(n["e"])("label",{class:"block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300"},"Copyright",-1),h=["value"],y={class:"mb-6"},k=Object(n["e"])("label",{class:"block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300"},"Composition or derivative work?",-1),x=["onUpdate:modelValue"],_=Object(n["g"])(" %royalty "),F=["onUpdate:modelValue"],N={class:"mb-6"},T={class:"mb-6"},I={class:"text-sm"},A=Object(n["e"])("b",null,"Note",-1),E=Object(n["e"])("hr",{class:"mb-6"},null,-1),S=Object(n["e"])("h2",{class:"font-bold"},"View NFT",-1),C={class:"col-span-3"},D=Object(n["e"])("h1",{class:"font-bold text-center"},"ArtPay Escrow",-1),M=Object(n["e"])("h2",{class:"font-bold"},"Create Escrow",-1),R=Object(n["e"])("div",{class:"py-2"},"Contractor Address",-1),P=Object(n["e"])("div",{class:"py-2"},"Project Title",-1),U=Object(n["e"])("div",{class:"py-2"},"Type",-1),V=Object(n["e"])("option",{value:"1"},"Custom Artwork",-1),L=Object(n["e"])("option",{value:"2"},"Artwork 1",-1),B=Object(n["e"])("option",{value:"3"},"Free Artwork",-1),W=Object(n["e"])("option",{value:"4"},"Artwork Custom",-1),G=[V,L,B,W],H=Object(n["e"])("div",{class:"py-2"},"Description",-1),q=Object(n["e"])("div",{class:"py-2"},"Requirements",-1),Y=Object(n["e"])("div",{class:"py-2"},"Expiry Date",-1),X=Object(n["e"])("div",{class:"py-2"},"Locked Funds (Near)",-1),z=Object(n["e"])("h2",{class:"font-bold"},"setNFTDeliverable",-1),J=Object(n["e"])("div",{class:"py-2"},"Client",-1),K=Object(n["e"])("div",{class:"py-2"},"Escrow Id",-1),Q=Object(n["e"])("div",{class:"py-2"},"NFT Address",-1),Z=Object(n["e"])("div",{class:"py-2"},"Token TokenId",-1),$=Object(n["e"])("p",{class:"text-sm"},"This will transfer owner_id of NFT to escrow.artpay.testnet",-1),ee=Object(n["e"])("h2",{class:"font-bold"},"View Escrow",-1),te={for:"checkbox"},re=Object(n["e"])("div",{class:"py-2"},"Other Address",-1),ne=Object(n["e"])("div",{class:"py-2"},"Escrow ID",-1),ce=Object(n["e"])("hr",null,null,-1);function ae(e,t,r,c,a,V){var L=Object(n["n"])("nav-bar");return Object(n["k"])(),Object(n["d"])("div",o,[Object(n["h"])(L),Object(n["e"])("h1",s,Object(n["o"])(a.msg),1),Object(n["e"])("div",l," Message: "+Object(n["o"])(a.globalMessage),1),Object(n["e"])("div",i,[Object(n["e"])("div",u,[Object(n["e"])("div",d,[b,a.accountId?(Object(n["k"])(),Object(n["d"])(n["a"],{key:1},[Object(n["e"])("button",{onClick:t[0]||(t[0]=function(e){return V.getMetaData()}),class:Object(n["j"])(V.btnStyle)}," getMetaData ",2),p,Object(n["e"])("div",g,[m,Object(n["t"])(Object(n["e"])("input",{"onUpdate:modelValue":t[1]||(t[1]=function(e){return a.newNFT.tokenId=e}),type:"text",class:Object(n["j"])(V.inputStyle)},null,2),[[n["r"],a.newNFT.tokenId]])]),Object(n["e"])("div",f,[j,Object(n["t"])(Object(n["e"])("textarea",{"onUpdate:modelValue":t[2]||(t[2]=function(e){return a.newNFT.copyright=e}),row:"4",class:Object(n["j"])(V.inputStyle)},null,2),[[n["r"],a.newNFT.copyright]])]),Object(n["e"])("div",v,[O,Object(n["t"])(Object(n["e"])("select",{"onUpdate:modelValue":t[3]||(t[3]=function(e){return a.newNFT.rightAssign=e}),class:Object(n["j"])(V.inputStyle)},[(Object(n["k"])(!0),Object(n["d"])(n["a"],null,Object(n["m"])(a.rightAssign,(function(e,t){return Object(n["k"])(),Object(n["d"])("option",{key:t,value:e},Object(n["o"])(e),9,h)})),128))],2),[[n["q"],a.newNFT.rightAssign]])]),Object(n["e"])("div",y,[k,(Object(n["k"])(!0),Object(n["d"])(n["a"],null,Object(n["m"])(a.newNFT.attributeParties,(function(r,c){return Object(n["k"])(),Object(n["d"])("div",{key:c},[Object(n["t"])(Object(n["e"])("input",{"onUpdate:modelValue":function(e){return r.address=e},class:Object(n["j"])(V.inputStyle)},null,10,x),[[n["r"],r.address]]),_,Object(n["t"])(Object(n["e"])("input",{"onUpdate:modelValue":function(e){return r.value=e},class:Object(n["j"])(V.inputStyle),type:"number",min:"0",max:"100"},null,10,F),[[n["r"],r.value]]),Object(n["e"])("button",{onClick:t[4]||(t[4]=function(t){return V.deleteParty(e.index)}),class:Object(n["j"])(V.btnStyle)}," delete ",2)])})),128))]),Object(n["e"])("div",N,[Object(n["e"])("button",{onClick:t[5]||(t[5]=function(){return V.addParty&&V.addParty.apply(V,arguments)}),class:Object(n["j"])(V.btnStyle)}," Add Party ",2)]),Object(n["e"])("div",T,[Object(n["e"])("button",{onClick:t[6]||(t[6]=function(e){return V.mintNFT()}),class:Object(n["j"])(V.btnStyle)}," mintNFT ",2),Object(n["e"])("p",I,[A,Object(n["g"])(" Minting will incur "+Object(n["o"])(a.MINT_PRICE)+" yoctoNEAR. Reason: For storage",1)])]),E,S,Object(n["t"])(Object(n["e"])("input",{"onUpdate:modelValue":t[7]||(t[7]=function(e){return a.viewNFT.tokenId=e}),class:Object(n["j"])(V.inputStyle),type:"text"},null,2),[[n["r"],a.viewNFT.tokenId]]),Object(n["e"])("button",{onClick:t[8]||(t[8]=function(e){return V.viewNFTToken()}),class:Object(n["j"])(V.btnStyle)}," viewNFTToken ",2)],64)):(Object(n["k"])(),Object(n["d"])(n["a"],{key:0},[w],64))]),Object(n["e"])("div",C,[D,M,R,Object(n["t"])(Object(n["e"])("input",{"onUpdate:modelValue":t[9]||(t[9]=function(e){return a.newEscrowFormData.contractor=e}),class:Object(n["j"])(V.inputStyle),placeholder:"contractor address",type:"text"},null,2),[[n["r"],a.newEscrowFormData.contractor]]),P,Object(n["t"])(Object(n["e"])("input",{"onUpdate:modelValue":t[10]||(t[10]=function(e){return a.newEscrowFormData.title=e}),class:Object(n["j"])(V.inputStyle),placeholder:"Title",type:"text"},null,2),[[n["r"],a.newEscrowFormData.title]]),U,Object(n["t"])(Object(n["e"])("select",{"onUpdate:modelValue":t[11]||(t[11]=function(e){return a.newEscrowFormData.escrowType=e}),class:Object(n["j"])(V.inputStyle)},G,2),[[n["q"],a.newEscrowFormData.escrowType]]),H,Object(n["t"])(Object(n["e"])("input",{"onUpdate:modelValue":t[12]||(t[12]=function(e){return a.newEscrowFormData.description=e}),class:Object(n["j"])(V.inputStyle),placeholder:"Description",type:"text"},null,2),[[n["r"],a.newEscrowFormData.description]]),q,Object(n["t"])(Object(n["e"])("input",{"onUpdate:modelValue":t[13]||(t[13]=function(e){return a.newEscrowFormData.description=e}),class:Object(n["j"])(V.inputStyle),placeholder:"Description",type:"text"},null,2),[[n["r"],a.newEscrowFormData.description]]),Y,Object(n["t"])(Object(n["e"])("input",{"onUpdate:modelValue":t[14]||(t[14]=function(e){return a.newEscrowFormData.timestamp=e}),class:Object(n["j"])(V.inputStyle),placeholder:"Expiry Date",type:"date"},null,2),[[n["r"],a.newEscrowFormData.timestamp]]),X,Object(n["t"])(Object(n["e"])("input",{"onUpdate:modelValue":t[15]||(t[15]=function(e){return a.newEscrowFormData.lockedFunds=e}),class:Object(n["j"])(V.inputStyle),placeholder:"Funds in NEAR",type:"number",step:"0.01"},null,2),[[n["r"],a.newEscrowFormData.lockedFunds]]),Object(n["e"])("button",{onClick:t[16]||(t[16]=function(e){return V.createEscrow()}),class:Object(n["j"])(V.btnStyle)}," createEscrow ",2),z,J,Object(n["t"])(Object(n["e"])("input",{"onUpdate:modelValue":t[17]||(t[17]=function(e){return a.escrowDeliverable.client=e}),class:Object(n["j"])(V.inputStyle),placeholder:"NFT Address",type:"text"},null,2),[[n["r"],a.escrowDeliverable.client]]),K,Object(n["t"])(Object(n["e"])("input",{"onUpdate:modelValue":t[18]||(t[18]=function(e){return a.escrowDeliverable.escrowId=e}),class:Object(n["j"])(V.inputStyle),placeholder:"TokenId for NFT",type:"text"},null,2),[[n["r"],a.escrowDeliverable.escrowId]]),Q,Object(n["t"])(Object(n["e"])("input",{"onUpdate:modelValue":t[19]||(t[19]=function(e){return a.escrowDeliverable.nftAddress=e}),class:Object(n["j"])(V.inputStyle),placeholder:"NFT Address",type:"text"},null,2),[[n["r"],a.escrowDeliverable.nftAddress]]),Z,Object(n["t"])(Object(n["e"])("input",{"onUpdate:modelValue":t[20]||(t[20]=function(e){return a.escrowDeliverable.tokenId=e}),class:Object(n["j"])(V.inputStyle),placeholder:"TokenId for NFT",type:"text"},null,2),[[n["r"],a.escrowDeliverable.tokenId]]),Object(n["e"])("button",{onClick:t[21]||(t[21]=function(e){return V.setNFTDeliverable()}),class:Object(n["j"])(V.btnStyle)}," setNFTDeliverable ",2),$,ee,Object(n["t"])(Object(n["e"])("input",{type:"checkbox",id:"checkbox","onUpdate:modelValue":t[22]||(t[22]=function(e){return a.viewEscrow.you=e})},null,512),[[n["p"],a.viewEscrow.you]]),Object(n["e"])("label",te,"Are you client of escrow? "+Object(n["o"])(a.viewEscrow.you),1),re,Object(n["t"])(Object(n["e"])("input",{"onUpdate:modelValue":t[23]||(t[23]=function(e){return a.viewEscrow.client=e}),class:Object(n["j"])(V.inputStyle),placeholder:"Client escrow",type:"text"},null,2),[[n["r"],a.viewEscrow.client]]),ne,Object(n["t"])(Object(n["e"])("input",{"onUpdate:modelValue":t[24]||(t[24]=function(e){return a.viewEscrow.tokenId=e}),class:Object(n["j"])(V.inputStyle),placeholder:"Escrow Id",type:"text"},null,2),[[n["r"],a.viewEscrow.tokenId]]),Object(n["e"])("button",{onClick:t[25]||(t[25]=function(e){return V.viewEscrowToken()}),class:Object(n["j"])(V.btnStyle)}," viewEscrow ",2),a.viewEscrow.you?(Object(n["k"])(),Object(n["d"])("button",{key:0,onClick:t[26]||(t[26]=function(e){return V.approveClient()}),class:Object(n["j"])(V.btnStyle)}," approveClient ",2)):(Object(n["k"])(),Object(n["d"])("button",{key:1,onClick:t[27]||(t[27]=function(e){return V.approveContractor()}),class:Object(n["j"])(V.btnStyle)}," approveContractor ",2)),Object(n["e"])("button",{onClick:t[28]||(t[28]=function(e){return V.releaseEscrow()}),class:Object(n["j"])(V.btnStyle)}," releaseEscrow ",2),ce,Object(n["e"])("button",{onClick:t[29]||(t[29]=function(e){return V.viewEscrowAsClient()}),class:Object(n["j"])(V.btnStyle)}," viewEscrowAsClient ",2),Object(n["e"])("button",{onClick:t[30]||(t[30]=function(e){return V.viewEscrowAsContractor()}),class:Object(n["j"])(V.btnStyle)}," viewEscrowAsContractor ",2)])])])])}var oe=r("b85c"),se=(r("fb6a"),r("d3b7"),r("3ca3"),r("ddb0"),r("9861"),r("ac1f"),r("841c"),r("25f0"),r("a9e3"),r("a434"),{class:"bg-white border-gray-200 px-2 sm:px-4 py-2.5 rounded dark:bg-gray-800"}),le={class:"container flex flex-wrap justify-between items-center mx-auto"},ie=Object(n["e"])("a",{href:"#",class:"flex"},[Object(n["e"])("span",{class:"self-center text-lg font-semibold whitespace-nowrap dark:text-white"},"ArtPay")],-1),ue={class:"flex md:order-2"},de=Object(n["g"])(" Connect to NEAR Wallet "),be=Object(n["f"])('<button data-collapse-toggle="mobile-menu-4" type="button" class="inline-flex items-center p-2 text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600" aria-controls="mobile-menu-4" aria-expanded="false"><span class="sr-only">Open main menu</span><svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path></svg><svg class="hidden w-6 h-6" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg></button>',1),we=Object(n["e"])("div",{class:"hidden justify-between items-center w-full md:flex md:w-auto md:order-1",id:"mobile-menu-4"},[Object(n["e"])("ul",{class:"flex flex-col mt-4 md:flex-row md:space-x-8 md:mt-0 md:text-sm md:font-medium"},[Object(n["e"])("li",null,[Object(n["e"])("a",{href:"#",class:"block py-2 pr-4 pl-3 text-gray-700 border-b border-gray-100 hover:bg-gray-50 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 md:dark:hover:text-white dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent dark:border-gray-700"},"NFT")]),Object(n["e"])("li",null,[Object(n["e"])("a",{href:"#",class:"block py-2 pr-4 pl-3 text-gray-700 border-b border-gray-100 hover:bg-gray-50 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 md:dark:hover:text-white dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent dark:border-gray-700"},"Escrow")])])],-1);function pe(e,t,r,c,a,o){return Object(n["k"])(),Object(n["d"])("nav",se,[Object(n["e"])("div",le,[ie,Object(n["e"])("div",ue,[Object(n["e"])("button",{type:"button",class:"text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center mr-3 md:mr-0 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800",onClick:t[0]||(t[0]=function(e){return o.login()})},[null!==this.accountId?(Object(n["k"])(),Object(n["d"])(n["a"],{key:0},[Object(n["g"])(Object(n["o"])(a.accountId),1)],64)):(Object(n["k"])(),Object(n["d"])(n["a"],{key:1},[de],64))]),be]),we])])}var ge=r("5502"),me=new ge["a"].Store({state:{MAX_GAS:"300000000000000",MINT_PRICE:"10000000000000000000000",msg:"ArtPay",near:null,accountId:null,wallet:null,count:0,abi:{nft:{contractAddr:"nft.artpay.testnet",viewMethods:["nft_metadata","nft_token"],changeMethods:["nft_mint","allowance","nft_transfer"]},escrow:{contractAddr:"escrow.artpay.testnet",viewMethods:["get_escrow","get_escrow_new","get_escrows_as_contractor","get_escrows_as_client"],changeMethods:["release_escrow","contractor_approval","client_approval","create_new_escrow","take_my_money","set_deliverable","set_nft_deliverable"]}}},mutations:{setNear:function(e,t){!e.near&&(e.near=t)},setNearWallet:function(e,t){!e.wallet&&(e.wallet=t)},setAccountId:function(e,t){!e.accountId&&(e.accountId=t)},subtract:function(e,t){t?e.count-=t:e.count--}},actions:{addThreeAsync:function(e){var t=e.commit;setTimeout((function(){return t("add",3)}),3e3)}}}),fe={name:"NavBar",data:function(){return{accountId:null}},methods:{login:function(){var e=this;return Object(a["a"])(regeneratorRuntime.mark((function t(){return regeneratorRuntime.wrap((function(t){while(1)switch(t.prev=t.next){case 0:if(!me.state.wallet||!me.state.wallet.isSignedIn()){t.next=4;break}e.accountId=me.state.accountId,t.next=6;break;case 4:return t.next=6,me.state.wallet.requestSignIn(me.state.abi.nft.contractAddr,"ArtPay");case 6:case"end":return t.stop()}}),t)})))()}},created:function(){var e=this;return Object(a["a"])(regeneratorRuntime.mark((function t(){return regeneratorRuntime.wrap((function(t){while(1)switch(t.prev=t.next){case 0:me.state.wallet&&me.state.wallet.isSignedIn()&&(console.log("Calling AcountId"),e.accountId=me.state.accountId,console.log(e.accountId));case 1:case"end":return t.stop()}}),t)})))()}},je=r("d959"),ve=r.n(je);const Oe=ve()(fe,[["render",pe]]);var he=Oe,ye=r("d9eb"),ke=ye.connect,xe=ye.keyStores,_e=ye.WalletConnection,Fe=ye.utils,Ne=new xe.BrowserLocalStorageKeyStore(window.localStorage,"nearlib:keystore:");console.log(Ne);var Te={networkId:"testnet",keyStore:Ne,nodeUrl:"https://rpc.testnet.near.org",walletUrl:"https://wallet.testnet.near.org",helperUrl:"https://helper.testnet.near.org",explorerUrl:"https://explorer.testnet.near.org"},Ie={nft:{contractAddr:"nft.artpay.testnet",viewMethods:["nft_metadata","nft_token"],changeMethods:["nft_mint","allowance","nft_transfer"]},escrow:{contractAddr:"escrow.artpay.testnet",viewMethods:["get_escrow","get_escrow_new","get_escrows_as_contractor","get_escrows_as_client"],changeMethods:["release_escrow","contractor_approval","client_approval","create_new_escrow","take_my_money","set_deliverable","set_nft_deliverable"]}},Ae={name:"HelloWorld",components:{NavBar:he},data:function(){return{MAX_GAS:"300000000000000",MINT_PRICE:"10000000000000000000000",msg:"ArtPay",near:null,accountId:null,wallet:null,globalMessage:null,urlParams:null,rightAssign:["BYND","BYNC","BYSA","BYNCND","BYNCSA","FULL"],newNFT:{tokenId:"token-1",copyright:"Copyright © 2022 ArtPay all rights reserved. See URL for more information.",rightAssign:"BYND",attributeParties:[]},viewNFT:{tokenId:me.state.count},newEscrowFormData:{contractor:"example.tesnet",timestamp:(new Date).toISOString().slice(0,10),lockedFunds:0,title:"Project Title",description:"description",escrowType:"1",requirements:"Create NFT art of a donkey!"},escrowDeliverable:{client:"artpay.testnet",escrowId:"1",nftAddress:"nft.artpay.testnet",tokenId:"token-1"},viewEscrow:{you:!0,client:"artpay.testnet",tokenId:"1"}}},methods:{init:function(){var e=this;return Object(a["a"])(regeneratorRuntime.mark((function t(){return regeneratorRuntime.wrap((function(t){while(1)switch(t.prev=t.next){case 0:return e.urlParams=new URLSearchParams(window.location.search),t.next=3,ke(Te);case 3:e.near=t.sent,e.wallet=new _e(e.near),e.globalMessage="Transaction: ".concat(e.urlParams.has("transactionHashes")?e.urlParams.get("transactionHashes"):""),e.accountId=e.wallet.isSignedIn()&&e.wallet.getAccountId();case 7:case"end":return t.stop()}}),t)})))()},login:function(){return Object(a["a"])(regeneratorRuntime.mark((function e(){return regeneratorRuntime.wrap((function(e){while(1)switch(e.prev=e.next){case 0:return e.next=2,me.state.wallet.requestSignIn(Ie.nft.contractAddr,"ArtPay");case 2:case"end":return e.stop()}}),e)})))()},logout:function(){var e=this;return Object(a["a"])(regeneratorRuntime.mark((function t(){return regeneratorRuntime.wrap((function(t){while(1)switch(t.prev=t.next){case 0:e.wallet.signOut(),e.accountId=null;case 2:case"end":return t.stop()}}),t)})))()},getWallet:function(){var e=this;return Object(a["a"])(regeneratorRuntime.mark((function t(){var r;return regeneratorRuntime.wrap((function(t){while(1)switch(t.prev=t.next){case 0:return t.next=2,e.near.account(e.accountId);case 2:return r=t.sent,console.log(e.accountId),t.t0=console,t.next=7,r.getAccountDetails();case 7:t.t1=t.sent,t.t0.log.call(t.t0,t.t1);case 9:case"end":return t.stop()}}),t)})))()},getMetaData:function(){var e=this;return Object(a["a"])(regeneratorRuntime.mark((function t(){var r;return regeneratorRuntime.wrap((function(t){while(1)switch(t.prev=t.next){case 0:return e.globalMessage="Getting ".concat(Ie.escrow.contractAddr," Metadata"),r=e.loadNFTContract("nft"),t.next=4,r.nft_metadata();case 4:e.globalMessage=t.sent;case 5:case"end":return t.stop()}}),t)})))()},viewNFTToken:function(){var e=this;return Object(a["a"])(regeneratorRuntime.mark((function t(){var r,n;return regeneratorRuntime.wrap((function(t){while(1)switch(t.prev=t.next){case 0:return e.globalMessage="Getting ".concat(e.viewNFT.tokenId," NFT"),r=e.loadNFTContract("nft"),t.next=4,r.nft_token({token_id:e.viewNFT.tokenId.toString()});case 4:n=t.sent,e.globalMessage=n||"".concat(e.viewNFT.tokenId," doesn't exist for NFT");case 6:case"end":return t.stop()}}),t)})))()},mintNFT:function(){var e=this;return Object(a["a"])(regeneratorRuntime.mark((function t(){var r,n,c,a,o,s;return regeneratorRuntime.wrap((function(t){while(1)switch(t.prev=t.next){case 0:r={},n=Object(oe["a"])(e.newNFT.attributeParties);try{for(n.s();!(c=n.n()).done;)a=c.value,r[a.address]=a.value}catch(l){n.e(l)}finally{n.f()}return console.log(e.newNFT.rightAssign),o=e.loadNFTContract("nft"),t.next=7,o.nft_mint({receiver_id:e.wallet.getAccountId(),token_id:e.newNFT.tokenId,metadata:{title:"ArtPay",description:"Example ArtPay NFT!",media:"IPFS URL / URL",copies:1,copyright:e.newNFT.copyright,right_assign:e.newNFT.rightAssign},perpetual_royalties:r},e.MAX_GAS,e.MINT_PRICE);case 7:s=t.sent,e.globalMessage="Token minted as: ".concat(s);case 9:case"end":return t.stop()}}),t)})))()},createEscrow:function(){var e=this;return Object(a["a"])(regeneratorRuntime.mark((function t(){var r,n;return regeneratorRuntime.wrap((function(t){while(1)switch(t.prev=t.next){case 0:return r=new Date(e.newEscrowFormData.timestamp),n=e.loadNFTContract("escrow"),t.next=4,n.create_new_escrow({contractor:e.newEscrowFormData.contractor,timestamp:r.getTime(),title:e.newEscrowFormData.title,description:e.newEscrowFormData.description,escrow_type:e.newEscrowFormData.escrowType,requirement:"IPFS documentation",license_code:"Licencse Code",license_desc:"Example Licence",license_url:"IPFS Link"},e.MAX_GAS,Fe.format.parseNearAmount(e.newEscrowFormData.lockedFunds.toString()));case 4:case"end":return t.stop()}}),t)})))()},viewEscrowAsClient:function(){var e=this;return Object(a["a"])(regeneratorRuntime.mark((function t(){var r;return regeneratorRuntime.wrap((function(t){while(1)switch(t.prev=t.next){case 0:return r=e.loadNFTContract("escrow"),t.next=3,r.get_escrows_as_client({account_id:e.wallet.getAccountId()});case 3:e.globalMessage=t.sent,console.log(e.globalMessage);case 5:case"end":return t.stop()}}),t)})))()},viewEscrowAsContractor:function(){var e=this;return Object(a["a"])(regeneratorRuntime.mark((function t(){var r;return regeneratorRuntime.wrap((function(t){while(1)switch(t.prev=t.next){case 0:return r=e.loadNFTContract("escrow"),t.next=3,r.get_escrows_as_contractor({account_id:e.wallet.getAccountId()});case 3:e.globalMessage=t.sent,console.log(e.globalMessage);case 5:case"end":return t.stop()}}),t)})))()},approveClient:function(){var e=this;return Object(a["a"])(regeneratorRuntime.mark((function t(){var r;return regeneratorRuntime.wrap((function(t){while(1)switch(t.prev=t.next){case 0:return r=e.loadNFTContract("escrow"),t.next=3,r.client_approval({contractor:e.viewEscrow.client,id:Number(e.viewEscrow.tokenId)});case 3:case"end":return t.stop()}}),t)})))()},approveContractor:function(){var e=this;return Object(a["a"])(regeneratorRuntime.mark((function t(){var r;return regeneratorRuntime.wrap((function(t){while(1)switch(t.prev=t.next){case 0:return r=e.loadNFTContract("escrow"),t.next=3,r.contractor_approval({client:e.viewEscrow.client,id:Number(e.viewEscrow.tokenId)});case 3:case"end":return t.stop()}}),t)})))()},setNFTDeliverable:function(){var e=this;return Object(a["a"])(regeneratorRuntime.mark((function t(){var r;return regeneratorRuntime.wrap((function(t){while(1)switch(t.prev=t.next){case 0:return r=e.loadNFTContract("escrow"),t.next=3,r.set_nft_deliverable({client:e.escrowDeliverable.client,id:Number(e.escrowDeliverable.escrowId),nft_address:e.escrowDeliverable.nftAddress,token_id:e.escrowDeliverable.tokenId.toString()},2e14,1);case 3:e.globalMessage=t.sent,e.globalMessage.then((function(e){console.log(e)}));case 5:case"end":return t.stop()}}),t)})))()},viewEscrowToken:function(){var e=this;return Object(a["a"])(regeneratorRuntime.mark((function t(){var r,n;return regeneratorRuntime.wrap((function(t){while(1)switch(t.prev=t.next){case 0:return e.globalMessage="Getting Escrow",r=e.loadNFTContract("escrow"),n={client:e.wallet.getAccountId(),contractor:e.viewEscrow.client,id:Number(e.viewEscrow.tokenId)},e.viewEscrow.you||(n={client:e.viewEscrow.client,contractor:e.wallet.getAccountId(),id:Number(e.viewEscrow.tokenId)}),t.next=6,r.get_escrow_new(n);case 6:e.globalMessage=t.sent;case 7:case"end":return t.stop()}}),t)})))()},releaseEscrow:function(){var e=this;return Object(a["a"])(regeneratorRuntime.mark((function t(){var r,n;return regeneratorRuntime.wrap((function(t){while(1)switch(t.prev=t.next){case 0:return r=e.loadNFTContract("escrow"),n={client:e.wallet.getAccountId(),contractor:e.viewEscrow.client,id:Number(e.viewEscrow.tokenId)},e.viewEscrow.you||(n={client:e.viewEscrow.client,contractor:e.wallet.getAccountId(),id:Number(e.viewEscrow.tokenId)}),t.next=5,r.release_escrow(n,2e14,1);case 5:case"end":return t.stop()}}),t)})))()},sendOneNear:function(){var e=this;return Object(a["a"])(regeneratorRuntime.mark((function t(){var r,n,c;return regeneratorRuntime.wrap((function(t){while(1)switch(t.prev=t.next){case 0:return e.globalMessage="Sending One NEAR ...",r=e.loadNFTContract("escrow"),n=Fe.format.parseNearAmount("1"),t.next=5,r.take_my_money({},e.MAX_GAS,n);case 5:c=t.sent,console.log(c);case 7:case"end":return t.stop()}}),t)})))()},loadNFTContract:function(e){return new ye.Contract(this.wallet.account(),Ie[e].contractAddr,{viewMethods:Ie[e].viewMethods,changeMethods:Ie[e].changeMethods,sender:this.wallet.getAccountId()})},addParty:function(){this.newNFT.attributeParties.push({address:"",value:0})},deleteParty:function(e){this.newNFT.attributeParties.splice(e,1)}},mounted:function(){this.init()},computed:{btnStyle:function(){return"mb-6 text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"},inputStyle:function(){return"bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"}}};const Ee=ve()(Ae,[["render",ae]]);var Se=Ee,Ce=r("d9eb"),De=Ce.connect,Me=Ce.keyStores,Re=Ce.WalletConnection,Pe=new Me.BrowserLocalStorageKeyStore(window.localStorage,"nearlib:keystore:");console.log(Pe);var Ue={networkId:"testnet",keyStore:Pe,nodeUrl:"https://rpc.testnet.near.org",walletUrl:"https://wallet.testnet.near.org",helperUrl:"https://helper.testnet.near.org",explorerUrl:"https://explorer.testnet.near.org"},Ve={name:"App",components:{HelloWorld:Se},mounted:function(){return Object(a["a"])(regeneratorRuntime.mark((function e(){return regeneratorRuntime.wrap((function(e){while(1)switch(e.prev=e.next){case 0:return e.t0=me,e.next=3,De(Ue);case 3:e.t1=e.sent,e.t0.commit.call(e.t0,"setNear",e.t1),me.commit("setNearWallet",new Re(me.state.near)),me.commit("setAccountId",me.state.wallet.getAccountId());case 7:case"end":return e.stop()}}),e)})))()}};r("0b7f");const Le=ve()(Ve,[["render",c]]);var Be=Le;r("ba8c");Object(n["b"])(Be).use(me).mount("#app")},ba8c:function(e,t,r){},fcea:function(e,t,r){}});
//# sourceMappingURL=app.2802f69f.js.map