class App extends React.Component<unknown, { units?: ACUnit[] }> {

    private timeoutHandle?: number;
    private isMounted = false;

    public constructor(props: unknown) {
        super(props);

        this.state = { units: undefined };
    }

    public async componentDidMount() {
        this.isMounted = true;

        await this.tick();
    }

    public render(): React.ReactNode {
        if (this.state.units === undefined) {
            return <div>Awaiting initial data..</div>;
        }

        return <div>
            <h1>Ac Units</h1>
            <div className="units">
                {this.state.units.map(i => <Unit unit={i} />)}
            </div>
        </div>;
    }

    public componentWillUnmount() {
        this.isMounted = false;

        if (this.timeoutHandle !== undefined) {
            clearTimeout(this.timeoutHandle);
            this.timeoutHandle = undefined;
        }
    }

    private async tick() {
        if (!this.isMounted) {
            return;
        }

        const fetchResult = await fetch('/api/units');
        const units = await fetchResult.json();

        this.setState({ units });

        if (this.isMounted) {
            this.timeoutHandle = setTimeout(this.tick.bind(this), 1_000);
        }
    }
}

class Unit extends React.Component<{ unit: ACUnit }> {
    public render(): React.ReactNode {
        return <div className="unit">
            <h2>{this.props.unit.name}</h2>
            <dl>
                {Object.entries(this.props.unit.state)
                    .flatMap(([key, value]) =>
                        [
                            <dt>{key}</dt>,
                            <dd>{value}</dd>,
                        ]
                    )}
            </dl>
        </div>;
    }
}

ReactDOM.render(<App />, document.body);

interface ACUnit {
    name: string;
    state: {
        power_status: string;
        mode: string;
        target_temperature: number;
        fan_mode: string;
        swing_mode: number;
        power_selection: number;
        merit_a: number;
        merit_b: number;
        air_pure_ion: number;
        indoor_temp: number;
        outdoor_temp: number;
        self_cleaning: number;
    };
}