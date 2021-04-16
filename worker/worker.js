addEventListener('fetch', event => {
	event.respondWith(handleRequest(event.request));
});
const D_MAXLEN = 127;
let MAXLEN;

const stat = n => fetch(`https://http.cat/${n}.jpg`).then(r => new Response(r, {status: n}));
async function handleRequest(req) {
	if (typeof QR_CODE !== "undefined") {
		MAXLEN ??= MAXLEN.then(_ => QR_CODE
			.get('max_len', {type: "text", cacheTtl: 3600})
			.then(v => v ?? D_MAXLEN)
			.then(Number)
		).catch(_ => D_MAXLEN)
	} else {
		MAXLEN = D_MAXLEN;
	}
	let max_len = await MAXLEN;
	if (req.url.length > max_len) {
		return stat(413);
	}
	const { handle_request } = wasm_bindgen;
	await wasm_bindgen(wasm);
	const output = handle_request(request.url);

	let res = new Response(output, { status: 200 });
	res.headers.set('Content-type', 'image/svg+xml');
	return res;
}
