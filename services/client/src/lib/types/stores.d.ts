export type UIStore = {
	businessSelected: number;
	isSimMoving: boolean;
	isChoosingPoint: boolean;
	onPosConfirmed?: (pos: [number, number]) => void;
};

export type ConfigStore = {
	movementSpeed: number;
	requestDebounceRadius: number;
	requestRadius: number;
	maxVisibleRadius: number;
};
