addEventListener('fetch', event => {
	event.respondWith(
		handleRequest(event.request)
		.then(v => {
			if (v instanceof Response) return v;
			return new Response(JSON.stringify(v));
		})
		.catch(e => new Response(JSON.stringify(e.stack), {status: 500}))
	);
});
const D_MAX = 127;
let MAXLEN = Promise.resolve(D_MAX);
if (typeof QR_CODE !== "undefined") {
	MAXLEN = MAXLEN.then(() => QR_CODE
		.get("max_len")
		.then(v => v ? Number(v) : D_MAX)
	).catch(_ => D_MAX);
}


const stat = async n => new Response(
	await fetch(`https://http.cat/${n}.jpg`),
	{status: n}
);
async function handleRequest(req) {
	if (req.url.length > (await MAXLEN)) return await stat(414);
	const { handle_request } = wasm_bindgen;
	await wasm_bindgen(wasm);
	const output = handle_request(req.url);

	return new Response(output, {
		status: 200,
		headers: {'content-type': 'image/svg+xml'},
	});
}
