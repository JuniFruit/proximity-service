import messageStore from '@/stores/message';
import type { BusinessData } from '@/types/business';
import type { Icon, LatLngExpression, MapOptions } from 'leaflet';

export const getUserGeo = (onSuccess: PositionCallback) => {
	if (window && window.navigator) {
		const watchId = window.navigator.geolocation.watchPosition(onSuccess, (err) =>
			messageStore.update(() => err.message)
		);
		return watchId;
	} else {
		messageStore.update(() => 'Your browser does not support geo location');
		return 0;
	}
};

export const initialMapOpts: MapOptions = {
	maxZoom: 18,
	minZoom: 5,
	zoomDelta: 0.5,
	preferCanvas: true,
	center: [39, -98],
	zoom: 5,
	attributionControl: false
};

export function setupIcons(L: any) {
	const icons = ['destination', 'location', 'navigation'];

	const result: Record<string, Icon> = {};

	icons.forEach((icon) => {
		result[icon] = L.icon({
			iconUrl: icon + '.png',
			iconSize: [34, 34],
			iconAnchor: [22, 34],
			popupAnchor: [-6, -34]
		});
	});

	return result;
}

export function createMap(container: HTMLElement, L: any) {
	const main = L.tileLayer(
		'https://{s}.basemaps.cartocdn.com/rastertiles/voyager/{z}/{x}/{y}{r}.png',
		{
			attribution: `&copy;<a href="https://www.openstreetmap.org/copyright" target="_blank">OpenStreetMap</a>,
	        &copy;<a href="https://carto.com/attributions" target="_blank">CARTO</a>`,
			subdomains: 'abcd'
		}
	);
	// const osm = L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png');
	//
	// const osmHOT = L.tileLayer('https://{s}.tile.openstreetmap.fr/hot/{z}/{x}/{y}.png', {
	// 	attribution:
	// 		'© OpenStreetMap contributors, Tiles style by Humanitarian OpenStreetMap Team hosted by OpenStreetMap France'
	// });

	const m = L.map(container, { ...initialMapOpts, layers: [main] });

	return m;
}

export function createBusinessPopup(businessInfo: BusinessData, isClosed = false) {
	return `
<div class="flex flex-col gap-2">
  <h3 class="font-semibold text-sm">${businessInfo.name}</h3>
  <h5 class="">Rating: ${businessInfo.stars}/5</h5>
  <span class="font-medium py-0">${businessInfo.type}</span>
  <span class="font-normal">${isClosed ? 'Opens at ' + businessInfo.opensAt + ':00' : 'Closes at ' + businessInfo.closesAt + ':00'}</span>
</div>
`;
}
/**
 * Find distance between 2 geo points in meters
 */
export function findDistanceBetweenPoints(pos1: LatLngExpression, pos2: LatLngExpression) {
	if (!Array.isArray(pos1) || !Array.isArray(pos2)) return 0;
	const KEquatorialRadiusInMetres = 6378137;
	const KPiDouble = 3.141592654;
	const KDegreesToRadiansDouble = KPiDouble / 180.0;

	const lon1 = pos1[1] * KDegreesToRadiansDouble;
	const lat1 = pos1[0] * KDegreesToRadiansDouble;
	const lon2 = pos2[1] * KDegreesToRadiansDouble;
	const lat2 = pos2[0] * KDegreesToRadiansDouble;
	const cosAngle =
		Math.sin(lat1) * Math.sin(lat2) + Math.cos(lat1) * Math.cos(lat2) * Math.cos(lon2 - lon1);

	/*
    Inaccurate trig functions can cause cos_angle to be a tiny amount
    greater than 1 if the two positions are very close. That in turn causes
    acos to give a domain error and return the special floating point value
    -1.#IND000000000000, meaning 'indefinite'. Observed on VS2008 on 64-bit Windows.
    */
	if (cosAngle >= 1) return 0;

	const angle = Math.acos(cosAngle);
	return angle * KEquatorialRadiusInMetres;
}
