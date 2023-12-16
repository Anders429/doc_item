const searchIndex = new Map(JSON.parse('[\
["custom_docbox",{"doc":"","t":"H","n":["foo"],"q":[[0,"custom_docbox"]],"d":[""],"i":[0],"f":[[[],1]],"c":[],"p":[[1,"tuple"]],"b":[]}],\
["doc_item",{"doc":"Attributes for item-level documentation customization.","t":"XXXX","n":["docbox","semi_transparent","short_docbox","since"],"q":[[0,"doc_item"]],"d":["Adds a docbox to the item’s item-info.","Makes an item semi-transparent in module lists.","Adds a short docbox to the item in module lists.","Adds a minimal version to an item."],"i":[0,0,0,0],"f":[0,0,0,0],"c":[],"p":[],"b":[]}],\
["experimental",{"doc":"","t":"H","n":["foo"],"q":[[0,"experimental"]],"d":["The docbox will indicate the function is experimental. It …"],"i":[0],"f":[[[],1]],"c":[],"p":[[1,"tuple"]],"b":[]}]\
]'));
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
