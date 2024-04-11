<script lang="ts">
	import { browser } from '$app/environment';
	import type { LatLngExpression, Map, Icon, Marker } from 'leaflet';
	import { onDestroy, onMount } from 'svelte';
	import { Footer } from '@/components/footer';
	import { Header } from '@/components/header';
	import { BusinessCarousel } from '@/components/business-carousel';
	import { searchBusinesses } from '@/shared/api';
	import type { BusinessData } from '@/types/business.d';
	import {
		createBusinessPopup,
		createMap,
		findDistanceBetweenPoints,
		getUserGeo,
		initialMapOpts,
		setupIcons
	} from '@/shared/general';
	import messageStore from '@/stores/message';
	import windowStore from '@/stores/window';
	import uiStore from '@/stores/ui';
	import type { UIStore } from '@/types/stores';

	let L: any;
	let map: Map;
	let mapContainer: HTMLDivElement;
	let businesses: Record<string, BusinessData> = {};
	let currentView: LatLngExpression = initialMapOpts.center!;
	let defaultZoom = 15.5;
	let watchPosId: number;
	let icons: Record<string, Icon>;
	let currentPosMarker: Marker;
	let maxVisibleRadius = 5000; // businesses that outside will be freed
	let movingInterval: number;
	let businessMarkers: Record<string, Marker> = {};
	let isTrackingView = false;
	let uiStoreData: UIStore;
	let prevSelectedBusiness: number = 0;

	uiStore.subscribe((data) => {
		uiStoreData = data;
	});

	$: {
		if (uiStoreData.businessSelected && uiStoreData.businessSelected !== prevSelectedBusiness) {
			isTrackingView = false;
			const key = uiStoreData.businessSelected;
			prevSelectedBusiness = key;
			if (businesses[key]) {
				const { lat, lon } = businesses[key];
				map.setView([lat, lon]);
			}
		}
	}

	function init(container: HTMLElement) {
		map = createMap(container, L);
		watchPosId = getUserGeo((pos) => onPosChanged(pos, true));
		icons = setupIcons(L);
		map.setZoom(defaultZoom);
		map.on('drag', () => {
			isTrackingView = false;
		});
	}

	function simulateMovement() {
		if (movingInterval) {
			clearInterval(movingInterval);
		}
		movingInterval = setInterval(() => {
			const updatedLat = (currentView as unknown as [number, number])[0] + 0.0001;
			const updatedLon = (currentView as unknown as [number, number])[1] + 0.0001;
			onPosChanged({
				coords: {
					latitude: updatedLat,
					longitude: updatedLon
				}
			});
		}, 1000);
	}

	function createBusinessMarker(data: BusinessData): Marker {
		const hours = new Date().getHours();
		const isOpenNow = hours >= data.opensAt && hours <= data.closesAt;
		return L.marker([data.lat, data.lon], {
			icon: icons.location,
			title: data.name,
			opacity: isOpenNow ? 1 : 0.5,
			riseOnHover: true
		})
			.bindPopup(createBusinessPopup(data, !isOpenNow))
			.addTo(map);
	}

	function onBusinessesFound(res?: BusinessData[]) {
		const result = res || [];
		if (!result.length && !Object.keys(businesses).length) {
			messageStore.update(() => 'Nothing was found in this area');
		}

		// normalize;
		const newBusinesses = { ...businesses };

		result.forEach((item) => {
			const key = item.id;
			if (!businessMarkers[key]) {
				businessMarkers[key] = createBusinessMarker(item);
			}

			newBusinesses[key] = item;
		});
		// remove items outside radius
		Object.keys(newBusinesses).forEach((key) => {
			const { lat, lon } = newBusinesses[key];
			const isOutside = findDistanceBetweenPoints(currentView, [lat, lon]) > maxVisibleRadius;
			if (isOutside) {
				businessMarkers[key].remove();
				delete businessMarkers[key];
				delete newBusinesses[key];
			}
		});

		businesses = newBusinesses;
	}

	function onPosChanged(pos: any, isSetView = false) {
		const lat = parseFloat(pos.coords.latitude);
		const lon = parseFloat(pos.coords.longitude);
		currentView = [lat, lon];
		searchBusinesses(currentView).then(onBusinessesFound);
		updateNavMarker();
		if (isTrackingView || isSetView) {
			map.setView(currentView);
		}
	}

	function setOnCurrentPos() {
		map.setView(currentView);
		isTrackingView = true;
	}

	function updateNavMarker() {
		if (currentPosMarker) {
			currentPosMarker.setLatLng(currentView);
		} else {
			currentPosMarker = L.marker(currentView, { icon: icons.navigation }).addTo(map);
		}
	}

	function onWindowResize() {
		const isMobile = window.innerWidth <= 800 && window.innerHeight <= 600;
		windowStore.update(() => isMobile);
	}

	onMount(async () => {
		if (browser) {
			L = await import('leaflet');
			if (mapContainer) {
				init(mapContainer);
			}
			window.addEventListener('resize', onWindowResize);
			onWindowResize();
		}
	});

	onDestroy(() => {
		if (watchPosId) {
			navigator.geolocation.clearWatch(watchPosId);
		}
		if (browser) {
			window.removeEventListener('resize', onWindowResize);
		}
		clearInterval(movingInterval);
	});
</script>

<svelte:head>
	<link
		rel="stylesheet"
		href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css"
		integrity="sha256-p4NxAoJBhIIN+hmNHrzRCf9tD/miZyoHS5obTRR9BMY="
		crossorigin=""
	/>
</svelte:head>
<div class="page_container">
	<div class="header_contianer">
		<Header on:findMe={setOnCurrentPos} />
	</div>
	<div id="map" bind:this={mapContainer} />
	<div class="carousel_container">
		<BusinessCarousel businesses={Object.values(businesses)} />
	</div>
	<div class="footer_container">
		<Footer on:simMove={simulateMovement} />
	</div>
</div>

<style scoped lang="postcss">
	#map {
		height: 95%;
		width: 100%;
		z-index: 10;
	}
	.page_container {
		@apply relative h-full w-full;
	}
	.footer_container {
		@apply absolute bottom-0 left-0 z-40 w-full;
	}
	.header_contianer {
		@apply absolute left-0 top-5 z-40 h-fit w-full;
	}
	.carousel_container {
		@apply absolute bottom-16 left-0 z-30 w-full px-2;
	}
</style>
