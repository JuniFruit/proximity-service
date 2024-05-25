export type WayNode = {
	type: string;
	lat: number;
	lon: number;
	id: number;
};

export type PathData = {
	area: number[]; // (lat, lon) of bounding box area where search is performed
	target: number[]; // (lat, lon)
	origin: number[]; // (lat, lon)
};
