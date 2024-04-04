<script lang="ts">
	import { browser } from '$app/environment';
	import type { LatLngExpression, Map, Icon, Marker, LayerGroup } from 'leaflet';
	import { onDestroy, onMount } from 'svelte';
	import { Footer } from '@/components/footer';
	import { Header } from '@/components/header';
	import { BusinessCarousel } from '@/components/business-carousel';
	import { searchBusinesses } from '@/shared/api';
	import type { BusinessData } from '@/types/business.d';
	import {
		createBusinessPopup,
		createMap,
		getUserGeo,
		initialMapOpts,
		setupIcons
	} from '@/shared/general';
	import messageStore from '@/stores/message';

	let L: any;
	let map: Map;
	let mapContainer: HTMLDivElement;
	let businesses: BusinessData[] = [];
	let currentView: LatLngExpression = initialMapOpts.center!;
	let defaultZoom = 16.5;
	let watchPosId: number;
	let icons: Record<string, Icon>;
	let currentPosMarker: Marker;
	let businessLayerGroup: LayerGroup;

	function init(container: HTMLElement) {
		map = createMap(container, L);
		watchPosId = getUserGeo(onPosChanged);
		icons = setupIcons(L);
	}

	function onBusinessesFound(res?: BusinessData[]) {
		const result = res || [];
		if (!result.length) {
			messageStore.update(() => 'Nothing was found in this area');
		}
		if (businessLayerGroup) {
			map.removeLayer(businessLayerGroup);
		}

		businesses = result;
		const markers: Marker[] = result.map((data) => {
			const hours = new Date().getHours();
			const isClosed = hours > data.opensAt && hours < data.closesAt;
			return L.marker([data.lat, data.lon], {
				icon: icons.location,
				title: data.name,
				opacity: isClosed ? 0.5 : 1,
				riseOnHover: true
			}).bindPopup(createBusinessPopup(data));
		});
		businessLayerGroup = L.layerGroup(markers);
		map.addLayer(businessLayerGroup);
	}

	function onPosChanged(pos: any) {
		const lat = pos.coords.latitude;
		const lon = pos.coords.longitude;
		currentView = [lat, lon];
		map.setView(currentView, defaultZoom);
		searchBusinesses(currentView).then(onBusinessesFound);
		updateNavMarker();
	}

	function setOnCurrentPos() {
		map.setView(currentView, defaultZoom);
	}

	function updateNavMarker() {
		if (currentPosMarker) {
			currentPosMarker.setLatLng(currentView);
		} else {
			currentPosMarker = L.marker(currentView, { icon: icons.navigation }).addTo(map);
		}
	}

	onMount(async () => {
		if (browser) {
			L = await import('leaflet');
			if (mapContainer) {
				init(mapContainer);
			}
		}
	});

	onDestroy(() => {
		if (watchPosId) {
			navigator.geolocation.clearWatch(watchPosId);
		}
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
		<BusinessCarousel {businesses} />
	</div>
	<div class="footer_container">
		<Footer />
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
