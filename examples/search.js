"use strict";(function(){const itemTypes=["mod","externcrate","import","struct","enum","fn","type","static","trait","impl","tymethod","method","structfield","variant","macro","primitive","associatedtype","constant","associatedconstant","union","foreigntype","keyword","existential","attr","derive","traitalias",];const TY_PRIMITIVE=itemTypes.indexOf("primitive");const TY_KEYWORD=itemTypes.indexOf("keyword");function printTab(nb){let iter=0;let foundCurrentTab=false;let foundCurrentResultSet=false;onEachLazy(document.getElementById("titles").childNodes,elem=>{if(nb===iter){addClass(elem,"selected");foundCurrentTab=true}else{removeClass(elem,"selected")}iter+=1});iter=0;onEachLazy(document.getElementById("results").childNodes,elem=>{if(nb===iter){addClass(elem,"active");foundCurrentResultSet=true}else{removeClass(elem,"active")}iter+=1});if(foundCurrentTab&&foundCurrentResultSet){searchState.currentTab=nb}else if(nb!=0){printTab(0)}}const levenshtein_row2=[];function levenshtein(s1,s2){if(s1===s2){return 0}const s1_len=s1.length,s2_len=s2.length;if(s1_len&&s2_len){let i1=0,i2=0,a,b,c,c2;const row=levenshtein_row2;while(i1<s1_len){row[i1]=++i1}while(i2<s2_len){c2=s2.charCodeAt(i2);a=i2;++i2;b=i2;for(i1=0;i1<s1_len;++i1){c=a+(s1.charCodeAt(i1)!==c2?1:0);a=row[i1];b=b<a?(b<c?b+1:c):(a<c?a+1:c);row[i1]=b}}return b}return s1_len+s2_len}window.initSearch=rawSearchIndex=>{const MAX_LEV_DISTANCE=3;const MAX_RESULTS=200;const GENERICS_DATA=2;const NAME=0;const INPUTS_DATA=0;const OUTPUT_DATA=1;const NO_TYPE_FILTER=-1;let searchIndex;let currentResults;const ALIASES={};const params=searchState.getQueryStringParams();if(searchState.input.value===""){searchState.input.value=params.search||""}function isWhitespace(c){return" \t\n\r".indexOf(c)!==-1}function isSpecialStartCharacter(c){return"<\"".indexOf(c)!==-1}function isEndCharacter(c){return",>-".indexOf(c)!==-1}function isStopCharacter(c){return isWhitespace(c)||isEndCharacter(c)}function isErrorCharacter(c){return"()".indexOf(c)!==-1}function itemTypeFromName(typename){for(let i=0,len=itemTypes.length;i<len;++i){if(itemTypes[i]===typename){return i}}throw new Error("Unknown type filter `"+typename+"`")}function getStringElem(query,parserState,isInGenerics){if(isInGenerics){throw new Error("`\"` cannot be used in generics")}else if(query.literalSearch){throw new Error("Cannot have more than one literal search element")}else if(parserState.totalElems-parserState.genericsElems>0){throw new Error("Cannot use literal search when there is more than one element")}parserState.pos+=1;const start=parserState.pos;const end=getIdentEndPosition(parserState);if(parserState.pos>=parserState.length){throw new Error("Unclosed `\"`")}else if(parserState.userQuery[end]!=="\""){throw new Error(`Unexpected \`${parserState.userQuery[end]}\` in a string element`)}else if(start===end){throw new Error("Cannot have empty string element")}parserState.pos+=1;query.literalSearch=true}function isPathStart(parserState){return parserState.userQuery.slice(parserState.pos,parserState.pos+2)=="::"}function isReturnArrow(parserState){return parserState.userQuery.slice(parserState.pos,parserState.pos+2)=="->"}function isIdentCharacter(c){return(c==="_"||(c>="0"&&c<="9")||(c>="a"&&c<="z")||(c>="A"&&c<="Z"))}function isSeparatorCharacter(c){return c===","||isWhitespaceCharacter(c)}function isWhitespaceCharacter(c){return c===" "||c==="\t"}function createQueryElement(query,parserState,name,generics,isInGenerics){if(name==="*"||(name.length===0&&generics.length===0)){return}if(query.literalSearch&&parserState.totalElems-parserState.genericsElems>0){throw new Error("You cannot have more than one element if you use quotes")}const pathSegments=name.split("::");if(pathSegments.length>1){for(let i=0,len=pathSegments.length;i<len;++i){const pathSegment=pathSegments[i];if(pathSegment.length===0){if(i===0){throw new Error("Paths cannot start with `::`")}else if(i+1===len){throw new Error("Paths cannot end with `::`")}throw new Error("Unexpected `::::`")}}}if(pathSegments.length===0||(pathSegments.length===1&&pathSegments[0]==="")){throw new Error("Found generics without a path")}parserState.totalElems+=1;if(isInGenerics){parserState.genericsElems+=1}return{name:name,fullPath:pathSegments,pathWithoutLast:pathSegments.slice(0,pathSegments.length-1),pathLast:pathSegments[pathSegments.length-1],generics:generics,}}function getIdentEndPosition(parserState){let end=parserState.pos;let foundExclamation=false;while(parserState.pos<parserState.length){const c=parserState.userQuery[parserState.pos];if(!isIdentCharacter(c)){if(c==="!"){if(foundExclamation){throw new Error("Cannot have more than one `!` in an ident")}else if(parserState.pos+1<parserState.length&&isIdentCharacter(parserState.userQuery[parserState.pos+1])){throw new Error("`!` can only be at the end of an ident")}foundExclamation=true}else if(isErrorCharacter(c)){throw new Error(`Unexpected \`${c}\``)}else if(isStopCharacter(c)||isSpecialStartCharacter(c)||isSeparatorCharacter(c)){break}else if(c===":"){if(!isPathStart(parserState)){break}parserState.pos+=1;foundExclamation=false}else{throw new Error(`Unexpected \`${c}\``)}}parserState.pos+=1;end=parserState.pos}return end}function getNextElem(query,parserState,elems,isInGenerics){const generics=[];let start=parserState.pos;let end;if(parserState.userQuery[parserState.pos]==="\""){start+=1;getStringElem(query,parserState,isInGenerics);end=parserState.pos-1}else{end=getIdentEndPosition(parserState)}if(parserState.pos<parserState.length&&parserState.userQuery[parserState.pos]==="<"){if(isInGenerics){throw new Error("Unexpected `<` after `<`")}else if(start>=end){throw new Error("Found generics without a path")}parserState.pos+=1;getItemsBefore(query,parserState,generics,">")}if(start>=end&&generics.length===0){return}elems.push(createQueryElement(query,parserState,parserState.userQuery.slice(start,end),generics,isInGenerics))}function getItemsBefore(query,parserState,elems,endChar){let foundStopChar=true;while(parserState.pos<parserState.length){const c=parserState.userQuery[parserState.pos];if(c===endChar){break}else if(isSeparatorCharacter(c)){parserState.pos+=1;foundStopChar=true;continue}else if(c===":"&&isPathStart(parserState)){throw new Error("Unexpected `::`: paths cannot start with `::`")}else if(c===":"||isEndCharacter(c)){let extra="";if(endChar===">"){extra="`<`"}else if(endChar===""){extra="`->`"}throw new Error("Unexpected `"+c+"` after "+extra)}if(!foundStopChar){if(endChar!==""){throw new Error(`Expected \`,\`, \` \` or \`${endChar}\`, found \`${c}\``)}throw new Error(`Expected \`,\` or \` \`, found \`${c}\``)}const posBefore=parserState.pos;getNextElem(query,parserState,elems,endChar===">");if(posBefore===parserState.pos){parserState.pos+=1}foundStopChar=false}parserState.pos+=1}function checkExtraTypeFilterCharacters(parserState){const query=parserState.userQuery;for(let pos=0;pos<parserState.pos;++pos){if(!isIdentCharacter(query[pos])&&!isWhitespaceCharacter(query[pos])){throw new Error(`Unexpected \`${query[pos]}\` in type filter`)}}}function parseInput(query,parserState){let c,before;let foundStopChar=true;while(parserState.pos<parserState.length){c=parserState.userQuery[parserState.pos];if(isStopCharacter(c)){foundStopChar=true;if(isSeparatorCharacter(c)){parserState.pos+=1;continue}else if(c==="-"||c===">"){if(isReturnArrow(parserState)){break}throw new Error(`Unexpected \`${c}\` (did you mean \`->\`?)`)}throw new Error(`Unexpected \`${c}\``)}else if(c===":"&&!isPathStart(parserState)){if(parserState.typeFilter!==null){throw new Error("Unexpected `:`")}if(query.elems.length===0){throw new Error("Expected type filter before `:`")}else if(query.elems.length!==1||parserState.totalElems!==1){throw new Error("Unexpected `:`")}else if(query.literalSearch){throw new Error("You cannot use quotes on type filter")}checkExtraTypeFilterCharacters(parserState);parserState.typeFilter=query.elems.pop().name;parserState.pos+=1;parserState.totalElems=0;query.literalSearch=false;foundStopChar=true;continue}if(!foundStopChar){if(parserState.typeFilter!==null){throw new Error(`Expected \`,\`, \` \` or \`->\`, found \`${c}\``)}throw new Error(`Expected \`,\`, \` \`, \`:\` or \`->\`, found \`${c}\``)}before=query.elems.length;getNextElem(query,parserState,query.elems,false);if(query.elems.length===before){parserState.pos+=1}foundStopChar=false}while(parserState.pos<parserState.length){c=parserState.userQuery[parserState.pos];if(isReturnArrow(parserState)){parserState.pos+=2;getItemsBefore(query,parserState,query.returned,"");if(query.returned.length===0){throw new Error("Expected at least one item after `->`")}break}else{parserState.pos+=1}}}function newParsedQuery(userQuery){return{original:userQuery,userQuery:userQuery.toLowerCase(),typeFilter:NO_TYPE_FILTER,elems:[],returned:[],foundElems:0,literalSearch:false,error:null,}}function buildUrl(search,filterCrates){let extra="?search="+encodeURIComponent(search);if(filterCrates!==null){extra+="&filter-crate="+encodeURIComponent(filterCrates)}return getNakedUrl()+extra+window.location.hash}function getFilterCrates(){const elem=document.getElementById("crate-search");if(elem&&elem.value!=="All crates"&&hasOwnPropertyRustdoc(rawSearchIndex,elem.value)){return elem.value}return null}function parseQuery(userQuery){userQuery=userQuery.trim();const parserState={length:userQuery.length,pos:0,totalElems:0,genericsElems:0,typeFilter:null,userQuery:userQuery.toLowerCase(),};let query=newParsedQuery(userQuery);try{parseInput(query,parserState);if(parserState.typeFilter!==null){let typeFilter=parserState.typeFilter;if(typeFilter==="const"){typeFilter="constant"}query.typeFilter=itemTypeFromName(typeFilter)}}catch(err){query=newParsedQuery(userQuery);query.error=err.message;query.typeFilter=-1;return query}if(!query.literalSearch){query.literalSearch=parserState.totalElems>1}query.foundElems=query.elems.length+query.returned.length;return query}function createQueryResults(results_in_args,results_returned,results_others,parsedQuery){return{"in_args":results_in_args,"returned":results_returned,"others":results_others,"query":parsedQuery,}}function execQuery(parsedQuery,searchWords,filterCrates){const results_others={},results_in_args={},results_returned={};function transformResults(results){const duplicates={};const out=[];for(const result of results){if(result.id>-1){const obj=searchIndex[result.id];obj.lev=result.lev;const res=buildHrefAndPath(obj);obj.displayPath=pathSplitter(res[0]);obj.fullPath=obj.displayPath+obj.name;obj.fullPath+="|"+obj.ty;if(duplicates[obj.fullPath]){continue}duplicates[obj.fullPath]=true;obj.href=res[1];out.push(obj);if(out.length>=MAX_RESULTS){break}}}return out}function sortResults(results,isType){const userQuery=parsedQuery.userQuery;const ar=[];for(const entry in results){if(hasOwnPropertyRustdoc(results,entry)){const result=results[entry];result.word=searchWords[result.id];result.item=searchIndex[result.id]||{};ar.push(result)}}results=ar;if(results.length===0){return[]}results.sort((aaa,bbb)=>{let a,b;a=(aaa.word!==userQuery);b=(bbb.word!==userQuery);if(a!==b){return a-b}a=(aaa.lev);b=(bbb.lev);if(a!==b){return a-b}a=(aaa.item.crate!==window.currentCrate);b=(bbb.item.crate!==window.currentCrate);if(a!==b){return a-b}a=aaa.word.length;b=bbb.word.length;if(a!==b){return a-b}a=aaa.word;b=bbb.word;if(a!==b){return(a>b?+1:-1)}a=(aaa.index<0);b=(bbb.index<0);if(a!==b){return a-b}a=aaa.index;b=bbb.index;if(a!==b){return a-b}if((aaa.item.ty===TY_PRIMITIVE&&bbb.item.ty!==TY_KEYWORD)||(aaa.item.ty===TY_KEYWORD&&bbb.item.ty!==TY_PRIMITIVE)){return-1}if((bbb.item.ty===TY_PRIMITIVE&&aaa.item.ty!==TY_PRIMITIVE)||(bbb.item.ty===TY_KEYWORD&&aaa.item.ty!==TY_KEYWORD)){return 1}a=(aaa.item.desc==="");b=(bbb.item.desc==="");if(a!==b){return a-b}a=aaa.item.ty;b=bbb.item.ty;if(a!==b){return a-b}a=aaa.item.path;b=bbb.item.path;if(a!==b){return(a>b?+1:-1)}return 0});let nameSplit=null;if(parsedQuery.elems.length===1){const hasPath=typeof parsedQuery.elems[0].path==="undefined";nameSplit=hasPath?null:parsedQuery.elems[0].path}for(const result of results){if(result.dontValidate){continue}const name=result.item.name.toLowerCase(),path=result.item.path.toLowerCase(),parent=result.item.parent;if(!isType&&!validateResult(name,path,nameSplit,parent)){result.id=-1}}return transformResults(results)}function checkGenerics(row,elem,defaultLev){if(row.length<=GENERICS_DATA||row[GENERICS_DATA].length===0){return elem.generics.length===0?defaultLev:MAX_LEV_DISTANCE+1}else if(row[GENERICS_DATA].length>0&&row[GENERICS_DATA][0][NAME]===""){if(row.length>GENERICS_DATA){return checkGenerics(row[GENERICS_DATA][0],elem,defaultLev)}return elem.generics.length===0?defaultLev:MAX_LEV_DISTANCE+1}let elem_name;if(elem.generics.length>0&&row[GENERICS_DATA].length>=elem.generics.length){const elems=Object.create(null);for(const entry of row[GENERICS_DATA]){elem_name=entry[NAME];if(elem_name===""){if(checkGenerics(entry,elem,MAX_LEV_DISTANCE+1)!==0){return MAX_LEV_DISTANCE+1}continue}if(elems[elem_name]===undefined){elems[elem_name]=0}elems[elem_name]+=1}for(const generic of elem.generics){let match=null;if(elems[generic.name]){match=generic.name}else{for(elem_name in elems){if(!hasOwnPropertyRustdoc(elems,elem_name)){continue}if(elem_name===generic){match=elem_name;break}}}if(match===null){return MAX_LEV_DISTANCE+1}elems[match]-=1;if(elems[match]===0){delete elems[match]}}return 0}return MAX_LEV_DISTANCE+1}function checkIfInGenerics(row,elem){let lev=MAX_LEV_DISTANCE+1;for(const entry of row[GENERICS_DATA]){lev=Math.min(checkType(entry,elem,true),lev);if(lev===0){break}}return lev}function checkType(row,elem,literalSearch){if(row[NAME].length===0){if(row.length>GENERICS_DATA){return checkIfInGenerics(row,elem)}return MAX_LEV_DISTANCE+1}let lev=levenshtein(row[NAME],elem.name);if(literalSearch){if(lev!==0){if(elem.generics.length===0){const checkGeneric=(row.length>GENERICS_DATA&&row[GENERICS_DATA].length>0);if(checkGeneric&&row[GENERICS_DATA].findIndex(tmp_elem=>tmp_elem[NAME]===elem.name)!==-1){return 0}}return MAX_LEV_DISTANCE+1}else if(elem.generics.length>0){return checkGenerics(row,elem,MAX_LEV_DISTANCE+1)}return 0}else if(row.length>GENERICS_DATA){if(elem.generics.length===0){if(lev===0){return 0}lev=checkIfInGenerics(row,elem);return lev+0.5}else if(lev>MAX_LEV_DISTANCE){return checkIfInGenerics(row,elem)}else{const tmp_lev=checkGenerics(row,elem,lev);if(tmp_lev>MAX_LEV_DISTANCE){return MAX_LEV_DISTANCE+1}return(tmp_lev+lev)/2}}else if(elem.generics.length>0){return MAX_LEV_DISTANCE+1}return lev}function findArg(row,elem,typeFilter){let lev=MAX_LEV_DISTANCE+1;if(row&&row.type&&row.type[INPUTS_DATA]&&row.type[INPUTS_DATA].length>0){for(const input of row.type[INPUTS_DATA]){if(!typePassesFilter(typeFilter,input[1])){continue}lev=Math.min(lev,checkType(input,elem,parsedQuery.literalSearch));if(lev===0){return 0}}}return parsedQuery.literalSearch?MAX_LEV_DISTANCE+1:lev}function checkReturned(row,elem,typeFilter){let lev=MAX_LEV_DISTANCE+1;if(row&&row.type&&row.type.length>OUTPUT_DATA){let ret=row.type[OUTPUT_DATA];if(typeof ret[0]==="string"){ret=[ret]}for(const ret_ty of ret){if(!typePassesFilter(typeFilter,ret_ty[1])){continue}lev=Math.min(lev,checkType(ret_ty,elem,parsedQuery.literalSearch));if(lev===0){return 0}}}return parsedQuery.literalSearch?MAX_LEV_DISTANCE+1:lev}function checkPath(contains,ty){if(contains.length===0){return 0}let ret_lev=MAX_LEV_DISTANCE+1;const path=ty.path.split("::");if(ty.parent&&ty.parent.name){path.push(ty.parent.name.toLowerCase())}const length=path.length;const clength=contains.length;if(clength>length){return MAX_LEV_DISTANCE+1}for(let i=0;i<length;++i){if(i+clength>length){break}let lev_total=0;let aborted=false;for(let x=0;x<clength;++x){const lev=levenshtein(path[i+x],contains[x]);if(lev>MAX_LEV_DISTANCE){aborted=true;break}lev_total+=lev}if(!aborted){ret_lev=Math.min(ret_lev,Math.round(lev_total/clength))}}return ret_lev}function typePassesFilter(filter,type){if(filter<=NO_TYPE_FILTER||filter===type)return true;const name=itemTypes[type];switch(itemTypes[filter]){case"constant":return name==="associatedconstant";case"fn":return name==="method"||name==="tymethod";case"type":return name==="primitive"||name==="associatedtype";case"trait":return name==="traitalias"}return false}function createAliasFromItem(item){return{crate:item.crate,name:item.name,path:item.path,desc:item.desc,ty:item.ty,parent:item.parent,type:item.type,is_alias:true,}}function handleAliases(ret,query,filterCrates){const lowerQuery=query.toLowerCase();const aliases=[];const crateAliases=[];if(filterCrates!==null){if(ALIASES[filterCrates]&&ALIASES[filterCrates][lowerQuery]){const query_aliases=ALIASES[filterCrates][lowerQuery];for(const alias of query_aliases){aliases.push(createAliasFromItem(searchIndex[alias]))}}}else{Object.keys(ALIASES).forEach(crate=>{if(ALIASES[crate][lowerQuery]){const pushTo=crate===window.currentCrate?crateAliases:aliases;const query_aliases=ALIASES[crate][lowerQuery];for(const alias of query_aliases){pushTo.push(createAliasFromItem(searchIndex[alias]))}}})}const sortFunc=(aaa,bbb)=>{if(aaa.path<bbb.path){return 1}else if(aaa.path===bbb.path){return 0}return-1};crateAliases.sort(sortFunc);aliases.sort(sortFunc);const pushFunc=alias=>{alias.alias=query;const res=buildHrefAndPath(alias);alias.displayPath=pathSplitter(res[0]);alias.fullPath=alias.displayPath+alias.name;alias.href=res[1];ret.others.unshift(alias);if(ret.others.length>MAX_RESULTS){ret.others.pop()}};onEach(aliases,pushFunc);onEach(crateAliases,pushFunc)}function addIntoResults(results,fullId,id,index,lev){if(lev===0||(!parsedQuery.literalSearch&&lev<=MAX_LEV_DISTANCE)){if(results[fullId]!==undefined){const result=results[fullId];if(result.dontValidate||result.lev<=lev){return}}results[fullId]={id:id,index:index,dontValidate:parsedQuery.literalSearch,lev:lev,}}}function handleSingleArg(row,pos,elem,results_others,results_in_args,results_returned){if(!row||(filterCrates!==null&&row.crate!==filterCrates)){return}let lev,lev_add=0,index=-1;const fullId=row.id;const in_args=findArg(row,elem,parsedQuery.typeFilter);const returned=checkReturned(row,elem,parsedQuery.typeFilter);addIntoResults(results_in_args,fullId,pos,index,in_args);addIntoResults(results_returned,fullId,pos,index,returned);if(!typePassesFilter(parsedQuery.typeFilter,row.ty)){return}const searchWord=searchWords[pos];if(parsedQuery.literalSearch){if(searchWord===elem.name){addIntoResults(results_others,fullId,pos,-1,0)}return}if(elem.name.length===0){if(row.type!==null){lev=checkGenerics(row.type,elem,MAX_LEV_DISTANCE+1);addIntoResults(results_others,fullId,pos,index,lev)}return}if(elem.fullPath.length>1){lev=checkPath(elem.pathWithoutLast,row);if(lev>MAX_LEV_DISTANCE||(parsedQuery.literalSearch&&lev!==0)){return}else if(lev>0){lev_add=lev/10}}if(searchWord.indexOf(elem.pathLast)>-1||row.normalizedName.indexOf(elem.pathLast)>-1){if(!results_others[fullId]!==undefined){index=row.normalizedName.indexOf(elem.pathLast)}}lev=levenshtein(searchWord,elem.pathLast);if(lev>0&&elem.pathLast.length>2&&searchWord.indexOf(elem.pathLast)>-1){if(elem.pathLast.length<6){lev=1}else{lev=0}}lev+=lev_add;if(lev>MAX_LEV_DISTANCE){return}else if(index!==-1&&elem.fullPath.length<2){lev-=1}if(lev<0){lev=0}addIntoResults(results_others,fullId,pos,index,lev)}function handleArgs(row,pos,results){if(!row||(filterCrates!==null&&row.crate!==filterCrates)){return}let totalLev=0;let nbLev=0;function checkArgs(elems,callback){for(const elem of elems){const lev=callback(row,elem,NO_TYPE_FILTER);if(lev<=1){nbLev+=1;totalLev+=lev}else{return false}}return true}if(!checkArgs(parsedQuery.elems,findArg)){return}if(!checkArgs(parsedQuery.returned,checkReturned)){return}if(nbLev===0){return}const lev=Math.round(totalLev/nbLev);addIntoResults(results,row.id,pos,0,lev)}function innerRunQuery(){let elem,i,nSearchWords,in_returned,row;if(parsedQuery.foundElems===1){if(parsedQuery.elems.length===1){elem=parsedQuery.elems[0];for(i=0,nSearchWords=searchWords.length;i<nSearchWords;++i){handleSingleArg(searchIndex[i],i,elem,results_others,results_in_args,results_returned)}}else if(parsedQuery.returned.length===1){elem=parsedQuery.returned[0];for(i=0,nSearchWords=searchWords.length;i<nSearchWords;++i){row=searchIndex[i];in_returned=checkReturned(row,elem,parsedQuery.typeFilter);addIntoResults(results_others,row.id,i,-1,in_returned)}}}else if(parsedQuery.foundElems>0){for(i=0,nSearchWords=searchWords.length;i<nSearchWords;++i){handleArgs(searchIndex[i],i,results_others)}}}if(parsedQuery.error===null){innerRunQuery()}const ret=createQueryResults(sortResults(results_in_args,true),sortResults(results_returned,true),sortResults(results_others,false),parsedQuery);handleAliases(ret,parsedQuery.original.replace(/"/g,""),filterCrates);if(parsedQuery.error!==null&&ret.others.length!==0){ret.query.error=null}return ret}function validateResult(name,path,keys,parent){if(!keys||!keys.length){return true}for(const key of keys){if(!(name.indexOf(key)>-1||path.indexOf(key)>-1||(parent!==undefined&&parent.name!==undefined&&parent.name.toLowerCase().indexOf(key)>-1)||levenshtein(name,key)<=MAX_LEV_DISTANCE)){return false}}return true}function nextTab(direction){const next=(searchState.currentTab+direction+3)%searchState.focusedByTab.length;searchState.focusedByTab[searchState.currentTab]=document.activeElement;printTab(next);focusSearchResult()}function focusSearchResult(){const target=searchState.focusedByTab[searchState.currentTab]||document.querySelectorAll(".search-results.active a").item(0)||document.querySelectorAll("#titles > button").item(searchState.currentTab);if(target){target.focus()}}function buildHrefAndPath(item){let displayPath;let href;const type=itemTypes[item.ty];const name=item.name;let path=item.path;if(type==="mod"){displayPath=path+"::";href=window.rootPath+path.replace(/::/g,"/")+"/"+name+"/index.html"}else if(type==="import"){displayPath=item.path+"::";href=window.rootPath+item.path.replace(/::/g,"/")+"/index.html#reexport."+name}else if(type==="primitive"||type==="keyword"){displayPath="";href=window.rootPath+path.replace(/::/g,"/")+"/"+type+"."+name+".html"}else if(type==="externcrate"){displayPath="";href=window.rootPath+name+"/index.html"}else if(item.parent!==undefined){const myparent=item.parent;let anchor="#"+type+"."+name;const parentType=itemTypes[myparent.ty];let pageType=parentType;let pageName=myparent.name;if(parentType==="primitive"){displayPath=myparent.name+"::"}else if(type==="structfield"&&parentType==="variant"){const enumNameIdx=item.path.lastIndexOf("::");const enumName=item.path.substr(enumNameIdx+2);path=item.path.substr(0,enumNameIdx);displayPath=path+"::"+enumName+"::"+myparent.name+"::";anchor="#variant."+myparent.name+".field."+name;pageType="enum";pageName=enumName}else{displayPath=path+"::"+myparent.name+"::"}href=window.rootPath+path.replace(/::/g,"/")+"/"+pageType+"."+pageName+".html"+anchor}else{displayPath=item.path+"::";href=window.rootPath+item.path.replace(/::/g,"/")+"/"+type+"."+name+".html"}return[displayPath,href]}function escape(content){const h1=document.createElement("h1");h1.textContent=content;return h1.innerHTML}function pathSplitter(path){const tmp="<span>"+path.replace(/::/g,"::</span><span>");if(tmp.endsWith("<span>")){return tmp.slice(0,tmp.length-6)}return tmp}function addTab(array,query,display){let extraClass="";if(display===true){extraClass=" active"}const output=document.createElement("div");let length=0;if(array.length>0){output.className="search-results "+extraClass;array.forEach(item=>{const name=item.name;const type=itemTypes[item.ty];length+=1;let extra="";if(type==="primitive"){extra=" <i>(primitive type)</i>"}else if(type==="keyword"){extra=" <i>(keyword)</i>"}const link=document.createElement("a");link.className="result-"+type;link.href=item.href;const wrapper=document.createElement("div");const resultName=document.createElement("div");resultName.className="result-name";if(item.is_alias){const alias=document.createElement("span");alias.className="alias";const bold=document.createElement("b");bold.innerText=item.alias;alias.appendChild(bold);alias.insertAdjacentHTML("beforeend","<span class=\"grey\"><i>&nbsp;- see&nbsp;</i></span>");resultName.appendChild(alias)}resultName.insertAdjacentHTML("beforeend",item.displayPath+"<span class=\""+type+"\">"+name+extra+"</span>");wrapper.appendChild(resultName);const description=document.createElement("div");description.className="desc";const spanDesc=document.createElement("span");spanDesc.insertAdjacentHTML("beforeend",item.desc);description.appendChild(spanDesc);wrapper.appendChild(description);link.appendChild(wrapper);output.appendChild(link)})}else if(query.error===null){output.className="search-failed"+extraClass;output.innerHTML="No results :(<br/>"+"Try on <a href=\"https://duckduckgo.com/?q="+encodeURIComponent("rust "+query.userQuery)+"\">DuckDuckGo</a>?<br/><br/>"+"Or try looking in one of these:<ul><li>The <a "+"href=\"https://doc.rust-lang.org/reference/index.html\">Rust Reference</a> "+" for technical details about the language.</li><li><a "+"href=\"https://doc.rust-lang.org/rust-by-example/index.html\">Rust By "+"Example</a> for expository code examples.</a></li><li>The <a "+"href=\"https://doc.rust-lang.org/book/index.html\">Rust Book</a> for "+"introductions to language features and the language itself.</li><li><a "+"href=\"https://docs.rs\">Docs.rs</a> for documentation of crates released on"+" <a href=\"https://crates.io/\">crates.io</a>.</li></ul>"}return[output,length]}function makeTabHeader(tabNb,text,nbElems){if(searchState.currentTab===tabNb){return"<button class=\"selected\">"+text+" <div class=\"count\">("+nbElems+")</div></button>"}return"<button>"+text+" <div class=\"count\">("+nbElems+")</div></button>"}function showResults(results,go_to_first,filterCrates){const search=searchState.outputElement();if(go_to_first||(results.others.length===1&&getSettingValue("go-to-only-result")==="true"&&(!search.firstChild||search.firstChild.innerText!==searchState.loadingText))){const elem=document.createElement("a");elem.href=results.others[0].href;removeClass(elem,"active");document.body.appendChild(elem);elem.click();return}if(results.query===undefined){results.query=parseQuery(searchState.input.value)}currentResults=results.query.userQuery;const ret_others=addTab(results.others,results.query,true);const ret_in_args=addTab(results.in_args,results.query,false);const ret_returned=addTab(results.returned,results.query,false);let currentTab=searchState.currentTab;if((currentTab===0&&ret_others[1]===0)||(currentTab===1&&ret_in_args[1]===0)||(currentTab===2&&ret_returned[1]===0)){if(ret_others[1]!==0){currentTab=0}else if(ret_in_args[1]!==0){currentTab=1}else if(ret_returned[1]!==0){currentTab=2}}let crates="";if(window.ALL_CRATES.length>1){crates=" in <select id=\"crate-search\"><option value=\"All crates\">"+"All crates</option>";for(const c of window.ALL_CRATES){crates+=`<option value="${c}" ${c == filterCrates && "selected"}>${c}</option>`}crates+="</select>"}let typeFilter="";if(results.query.typeFilter!==NO_TYPE_FILTER){typeFilter=" (type: "+escape(itemTypes[results.query.typeFilter])+")"}let output="<div id=\"search-settings\">"+`<h1 class="search-results-title">Results for ${escape(results.query.userQuery)}`+`${typeFilter}</h1> in ${crates} </div>`;if(results.query.error!==null){output+=`<h3>Query parser error: "${results.query.error}".</h3>`;output+="<div id=\"titles\">"+makeTabHeader(0,"In Names",ret_others[1])+"</div>";currentTab=0}else if(results.query.foundElems<=1&&results.query.returned.length===0){output+="<div id=\"titles\">"+makeTabHeader(0,"In Names",ret_others[1])+makeTabHeader(1,"In Parameters",ret_in_args[1])+makeTabHeader(2,"In Return Types",ret_returned[1])+"</div>"}else{const signatureTabTitle=results.query.elems.length===0?"In Function Return Types":results.query.returned.length===0?"In Function Parameters":"In Function Signatures";output+="<div id=\"titles\">"+makeTabHeader(0,signatureTabTitle,ret_others[1])+"</div>";currentTab=0}const resultsElem=document.createElement("div");resultsElem.id="results";resultsElem.appendChild(ret_others[0]);resultsElem.appendChild(ret_in_args[0]);resultsElem.appendChild(ret_returned[0]);search.innerHTML=output;const crateSearch=document.getElementById("crate-search");if(crateSearch){crateSearch.addEventListener("input",updateCrate)}search.appendChild(resultsElem);searchState.showResults(search);const elems=document.getElementById("titles").childNodes;searchState.focusedByTab=[];let i=0;for(const elem of elems){const j=i;elem.onclick=()=>{printTab(j)};searchState.focusedByTab.push(null);i+=1}printTab(currentTab)}function search(e,forced){const params=searchState.getQueryStringParams();const query=parseQuery(searchState.input.value.trim());if(e){e.preventDefault()}if(!forced&&query.userQuery===currentResults){if(query.userQuery.length>0){putBackSearch()}return}let filterCrates=getFilterCrates();if(filterCrates===null&&params["filter-crate"]!==undefined){filterCrates=params["filter-crate"]}searchState.title="Results for "+query.original+" - Rust";if(browserSupportsHistoryApi()){const newURL=buildUrl(query.original,filterCrates);if(!history.state&&!params.search){history.pushState(null,"",newURL)}else{history.replaceState(null,"",newURL)}}showResults(execQuery(query,searchWords,filterCrates),params.go_to_first,filterCrates)}function buildIndex(rawSearchIndex){searchIndex=[];const searchWords=[];let i,word;let currentIndex=0;let id=0;for(const crate in rawSearchIndex){if(!hasOwnPropertyRustdoc(rawSearchIndex,crate)){continue}let crateSize=0;const crateCorpus=rawSearchIndex[crate];searchWords.push(crate);const crateRow={crate:crate,ty:1,name:crate,path:"",desc:crateCorpus.doc,parent:undefined,type:null,id:id,normalizedName:crate.indexOf("_")===-1?crate:crate.replace(/_/g,""),};id+=1;searchIndex.push(crateRow);currentIndex+=1;const itemTypes=crateCorpus.t;const itemNames=crateCorpus.n;const itemPaths=crateCorpus.q;const itemDescs=crateCorpus.d;const itemParentIdxs=crateCorpus.i;const itemFunctionSearchTypes=crateCorpus.f;const paths=crateCorpus.p;const aliases=crateCorpus.a;let len=paths.length;for(i=0;i<len;++i){paths[i]={ty:paths[i][0],name:paths[i][1]}}len=itemTypes.length;let lastPath="";for(i=0;i<len;++i){if(typeof itemNames[i]==="string"){word=itemNames[i].toLowerCase();searchWords.push(word)}else{word="";searchWords.push("")}const row={crate:crate,ty:itemTypes[i],name:itemNames[i],path:itemPaths[i]?itemPaths[i]:lastPath,desc:itemDescs[i],parent:itemParentIdxs[i]>0?paths[itemParentIdxs[i]-1]:undefined,type:itemFunctionSearchTypes[i],id:id,normalizedName:word.indexOf("_")===-1?word:word.replace(/_/g,""),};id+=1;searchIndex.push(row);lastPath=row.path;crateSize+=1}if(aliases){ALIASES[crate]={};for(const alias_name in aliases){if(!hasOwnPropertyRustdoc(aliases,alias_name)){continue}if(!hasOwnPropertyRustdoc(ALIASES[crate],alias_name)){ALIASES[crate][alias_name]=[]}for(const local_alias of aliases[alias_name]){ALIASES[crate][alias_name].push(local_alias+currentIndex)}}}currentIndex+=crateSize}return searchWords}function onSearchSubmit(e){e.preventDefault();searchState.clearInputTimeout();search()}function putBackSearch(){const search_input=searchState.input;if(!searchState.input){return}if(search_input.value!==""&&!searchState.isDisplayed()){searchState.showResults();if(browserSupportsHistoryApi()){history.replaceState(null,"",buildUrl(search_input.value,getFilterCrates()))}document.title=searchState.title}}function registerSearchEvents(){const searchAfter500ms=()=>{searchState.clearInputTimeout();if(searchState.input.value.length===0){if(browserSupportsHistoryApi()){history.replaceState(null,window.currentCrate+" - Rust",getNakedUrl()+window.location.hash)}searchState.hideResults()}else{searchState.timeout=setTimeout(search,500)}};searchState.input.onkeyup=searchAfter500ms;searchState.input.oninput=searchAfter500ms;document.getElementsByClassName("search-form")[0].onsubmit=onSearchSubmit;searchState.input.onchange=e=>{if(e.target!==document.activeElement){return}searchState.clearInputTimeout();setTimeout(search,0)};searchState.input.onpaste=searchState.input.onchange;searchState.outputElement().addEventListener("keydown",e=>{if(e.altKey||e.ctrlKey||e.shiftKey||e.metaKey){return}if(e.which===38){const previous=document.activeElement.previousElementSibling;if(previous){previous.focus()}else{searchState.focus()}e.preventDefault()}else if(e.which===40){const next=document.activeElement.nextElementSibling;if(next){next.focus()}const rect=document.activeElement.getBoundingClientRect();if(window.innerHeight-rect.bottom<rect.height){window.scrollBy(0,rect.height)}e.preventDefault()}else if(e.which===37){nextTab(-1);e.preventDefault()}else if(e.which===39){nextTab(1);e.preventDefault()}});searchState.input.addEventListener("keydown",e=>{if(e.which===40){focusSearchResult();e.preventDefault()}});searchState.input.addEventListener("focus",()=>{putBackSearch()});searchState.input.addEventListener("blur",()=>{searchState.input.placeholder=searchState.input.origPlaceholder});if(browserSupportsHistoryApi()){const previousTitle=document.title;window.addEventListener("popstate",e=>{const params=searchState.getQueryStringParams();document.title=previousTitle;currentResults=null;if(params.search&&params.search.length>0){searchState.input.value=params.search;search(e)}else{searchState.input.value="";searchState.hideResults()}})}window.onpageshow=()=>{const qSearch=searchState.getQueryStringParams().search;if(searchState.input.value===""&&qSearch){searchState.input.value=qSearch}search()}}function updateCrate(ev){if(ev.target.value==="All crates"){const params=searchState.getQueryStringParams();const query=searchState.input.value.trim();if(!history.state&&!params.search){history.pushState(null,"",buildUrl(query,null))}else{history.replaceState(null,"",buildUrl(query,null))}}currentResults=null;search(undefined,true)}const searchWords=buildIndex(rawSearchIndex);registerSearchEvents();function runSearchIfNeeded(){if(searchState.getQueryStringParams().search){search()}}runSearchIfNeeded()};if(window.searchIndex!==undefined){initSearch(window.searchIndex)}})()