<script lang="ts">
	import uiStore from '@/stores/ui';
	import configStore from '@/stores/config';
	import { browser } from '$app/environment';
	import type { LatLngExpression, Map, Icon, Marker, Polyline, Circle } from 'leaflet';
	import { onDestroy, onMount } from 'svelte';
	import { Footer } from '@/components/footer';
	import { Header } from '@/components/header';
	import { BusinessCarousel } from '@/components/business-carousel';
	import { getPath, searchBusinesses } from '@/shared/api';
	import { isOpenNow } from '@/utils';
	import type { BusinessData } from '@/types/business.d';
	import {
		createBusinessPopup,
		createGeoJSONCircle,
		createMap,
		findDistanceBetweenPoints,
		getBoundingBoxFromPolygon,
		getUserGeo,
		initialMapOpts,
		setupIcons
	} from '@/shared/general';
	import messageStore from '@/stores/message';
	import windowStore from '@/stores/window';
	import { Button } from '@/components/ui/button';
	import type { WayNode } from '@/types/api';
	import BusinessList from '@/components/business-list/business-list.svelte';

	type CurrentPath = {
		path: number[][];
		polyline: Polyline | null;
	};

	let L: any;
	let map: Map;
	let mapContainer: HTMLDivElement;
	let businesses: Record<string, BusinessData> = {};
	let currentView: LatLngExpression = initialMapOpts.center!;
	let defaultZoom = 14;
	let watchPosId: number;
	let icons: Record<string, Icon>;
	let currentPosMarker: Marker;
	let movingInterval: number;
	let businessMarkers: Record<string, Marker> = {};
	let isTrackingView = false;
	let prevSelectedBusiness: number = 0;
	let currentCircle: Circle;
	let targetMarker: Marker;
	let lastView: LatLngExpression = initialMapOpts.center!;
	let resizeDebounce: ReturnType<typeof setTimeout>;
	let mobileBusinessListOpen: boolean = false;
	let currentPath: CurrentPath = {
		path: [],
		polyline: null
	};

	$: {
		if ($uiStore.businessSelected && $uiStore.businessSelected !== prevSelectedBusiness) {
			isTrackingView = false;
			const key = $uiStore.businessSelected;
			prevSelectedBusiness = key;
			if (businesses[key]) {
				const { lat, lon } = businesses[key];
				businessMarkers[key].openPopup();
				map.setView([lat, lon]);
			}
		}
	}

	function init(container: HTMLElement) {
		map = createMap(container, L);
		watchPosId = getUserGeo((pos) => onPosChanged(pos, isTrackingView));
		icons = setupIcons(L);
		map.setZoom(defaultZoom);
		map.on('drag', () => {
			isTrackingView = false;
		});
	}

	async function goToPoint(target: LatLngExpression) {
		await requestPath(target);
		simulateMovement();
	}

	async function requestPath(target: LatLngExpression) {
		if (Array.isArray(target) && Array.isArray(currentView)) {
			const circle = createGeoJSONCircle(
				currentView as number[],
				$configStore.requestRadius / 1000
			);
			const area = getBoundingBoxFromPolygon(circle);

			const path = await getPath({
				area,
				origin: currentView as number[],
				target: target as number[]
			});
			if (path) {
				drawPath(path);
			}
		}
	}

	function drawPath(path: WayNode[]) {
		const coords = path.map((item) => {
			return [item.lat, item.lon];
		});
		const poly = L.polyline(coords).addTo(map);
		currentPath.path = coords;
		currentPath.polyline = poly;
	}

	function onSimMovementCalled() {
		if (movingInterval && $uiStore.isSimMoving) {
			clearInterval(movingInterval);
			targetMarker && targetMarker.remove();
			currentPath.polyline?.remove();
			currentPath.polyline = null;
			uiStore.update((data) => {
				data.isSimMoving = false;
				return data;
			});
			return;
		}

		uiStore.update((data) => {
			data.isChoosingPoint = true;
			if ($configStore.simMovementMode === 'fly') {
				data.onPosConfirmed = simulateFlying;
			} else {
				data.onPosConfirmed = goToPoint;
			}
			return data;
		});
	}

	function simulateMovement() {
		$uiStore.isSimMoving = true;
		movingInterval = setInterval(() => {
			const nodePos = currentPath.path.shift();
			if (!nodePos) {
				onSimMovementCalled();
				return;
			}
			onPosChanged({
				coords: {
					latitude: nodePos[0],
					longitude: nodePos[1]
				}
			});
			if (currentPath.polyline) {
				currentPath.polyline.setLatLngs(currentPath.path as any);
			}
		}, 1000);
	}

	function simulateFlying(target: [number, number]) {
		$uiStore.isSimMoving = true;
		movingInterval = setInterval(() => {
			const lat1 = (currentView as unknown as [number, number])[0];
			const lon1 = (currentView as unknown as [number, number])[1];
			const lat2 = target[0];
			const lon2 = target[1];
			const isClose = findDistanceBetweenPoints([lat1, lon1], [lat2, lon2]) < 10;
			if (isClose) {
				onSimMovementCalled();
				return;
			}
			const dlat = lat2 - lat1;
			const dlon = lon2 - lon1;
			const angle = Math.atan2(dlat, dlon); // range (-PI, PI]
			const updatedLat = lat1 + 0.0001 * Math.sin(angle);
			const updatedLon = lon1 + 0.0001 * Math.cos(angle);
			onPosChanged({
				coords: {
					latitude: updatedLat,
					longitude: updatedLon
				}
			});
		}, 1000);
	}

	function createBusinessMarker(data: BusinessData): Marker {
		const isOpen = isOpenNow(data.opensAt, data.closesAt);
		return L.marker([data.lat, data.lon], {
			icon: icons.location,
			title: data.name,
			opacity: isOpen ? 1 : 0.5,
			riseOnHover: true
		})
			.bindPopup(createBusinessPopup(data, !isOpen))
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
			const isOutside =
				findDistanceBetweenPoints(currentView, [lat, lon]) > $configStore.maxVisibleRadius;
			if (isOutside) {
				businessMarkers[key].remove();
				delete businessMarkers[key];
				delete newBusinesses[key];
			}
		});

		businesses = newBusinesses;
		lastView = currentView;
	}

	function onPosChanged(pos: any, isSetView = false) {
		const lat = parseFloat(pos.coords.latitude);
		const lon = parseFloat(pos.coords.longitude);
		currentView = [lat, lon];
		if (findDistanceBetweenPoints(lastView, currentView) >= $configStore.requestDebounceRadius) {
			searchBusinesses(currentView, $configStore.requestRadius).then(onBusinessesFound);
		}
		updateNavMarker();
		if (isTrackingView || isSetView) {
			map.setView(currentView);
		}
	}

	function onConfirmPos() {
		const center = map.getCenter();
		targetMarker = L.marker(center, { icon: icons.destination, riseOnHover: true }).addTo(map);
		uiStore.update((data) => {
			data.isChoosingPoint = false;
			data.onPosConfirmed?.([center.lat, center.lng]);
			return data;
		});
	}

	function onCancelPos() {
		$uiStore.isChoosingPoint = false;
	}

	function setOnCurrentPos() {
		map.setView(currentView);
		isTrackingView = true;
	}

	function updateNavMarker() {
		if (!currentCircle) {
			currentCircle = L.circle(currentView, {
				radius: $configStore.requestRadius,
				color: '#b6f4e3'
			}).addTo(map);
		} else {
			currentCircle.setLatLng(currentView);
			currentCircle.setStyle({});
		}

		if (currentPosMarker) {
			currentPosMarker.setLatLng(currentView);
		} else {
			currentPosMarker = L.marker(currentView, { icon: icons.navigation }).addTo(map);
		}
	}

	function onWindowResize() {
		clearTimeout(resizeDebounce);
		resizeDebounce = setTimeout(() => {
			const isMobile = window.innerWidth <= 800;
			windowStore.update(() => isMobile);
		}, 400);
	}

	function onLocationSelected(pos: CustomEvent<[number, number]>) {
		onPosChanged({ coords: { latitude: pos.detail[0], longitude: pos.detail[1] } }, true);
		if ($uiStore.isSimMoving) {
			onSimMovementCalled();
		}
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
			clearTimeout(resizeDebounce);
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
		<Header on:findMe={setOnCurrentPos} on:locationSelected={onLocationSelected} />
	</div>
	<div id="map" bind:this={mapContainer}>
		{#if $uiStore.isChoosingPoint}
			<div class="absolute left-1/2 top-1/2 z-[1000] -translate-x-1/2 -translate-y-1/2">
				<img src="./destination.png" alt="point" class="mb-6 h-8 w-8" />
			</div>
		{/if}
	</div>
	{#if !$windowStore}
		<div class="carousel_container">
			<BusinessCarousel businesses={Object.values(businesses)} />
		</div>
	{:else}
		<BusinessList
			on:dialogClosed={() => (mobileBusinessListOpen = false)}
			isDialogOpen={mobileBusinessListOpen}
			businesses={Object.values(businesses)}
		/>
	{/if}
	<div class="footer_container">
		<Footer
			on:businessListClicked={() => (mobileBusinessListOpen = !mobileBusinessListOpen)}
			on:simulateMovement={onSimMovementCalled}
		/>
	</div>
	{#if $uiStore.isChoosingPoint}
		<div class="confirm_container">
			<div
				class="flex max-w-[425] items-center justify-between gap-3 rounded-lg bg-white px-4 py-2 shadow-lg"
			>
				<Button on:click={onConfirmPos}>Confirm</Button>
				<Button on:click={onCancelPos} variant="secondary">Cancel</Button>
			</div>
		</div>
	{/if}
</div>

<style scoped lang="postcss">
	#map {
		height: 95%;
		width: 100%;
		z-index: 10;
	}
	.confirm_container {
		@apply absolute left-1/2 top-2/3 z-40;
		transform: translate(-50%, -0%);
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
