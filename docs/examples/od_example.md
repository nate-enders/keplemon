# Orbit Determination Examples

## Creating a New Orbit State

```python title="make_od.py"
import json
from keplemon.bodies import Observatory, Sensor, Satellite
from keplemon.estimation import Observation, BatchLeastSquares
from keplemon.time import Epoch
from keplemon.enums import TimeSystem, KeplerianType
from keplemon.elements import TopocentricElements, TLE

# Update your a priori state here
tle = TLE.from_lines(
    "1 41838U 16065A   25127.90338593  .00000067  00000+0  00000+0 0  9990",
    "2 41838   4.7224  77.5036 0005258 282.3676  13.3050  1.00272812 31141",
)
sat = Satellite.from_tle(tle)

# This example is built using UDL-formatted obs
with open("example-obs.json", "r") as f:
    json_obs = json.load(f)

keplemon_obs = []

# This can be updated per observation to reflect actual sensor capabilities
dummy_sensor = Sensor(name="Fake Sensor", angular_noise=0.001)

for json_ob in json_obs:

    # Keys here are only available because the input example data is UDL-formatted
    epoch = Epoch.from_iso(json_ob["obTime"], TimeSystem.UTC)
    lat = json_ob["senlat"]
    lon = json_ob["senlon"]
    alt = json_ob["senalt"]
    site = Observatory("Dummy Site", lat, lon, alt)
    observer_teme = site.get_state_at_epoch(epoch)

    # KepLemon expects TEME.  The `from_j2000` constructor will be needed for most observations
    teme_topo = TopocentricElements.from_j2000(epoch, json_ob["ra"], json_ob["declination"])

    ob = Observation(dummy_sensor, epoch, teme_topo, observer_teme.position)
    keplemon_obs.append(ob)


# Create the BLS object
bls = BatchLeastSquares(keplemon_obs, sat)

# You can override the output type if you'd like to use a more accurate model than basic SGP4
bls.output_type = KeplerianType.MeanBrouwerXP

# This will iterate until the weighted RMS tolerance is achieved OR max iterations is achieved
bls.solve()
```

## Plotting OD Results

```python title="plot_od.py"

# Add code to create a new orbit state as seen in the make_od.py example
...

# Be sure to add additional imports for plotting
import plotly.graph_objects as go  # type: ignore
import plotly.offline as pyo  # type: ignore

times, a_priori_errors, fit_errors = [], [], []
for ob in keplemon_obs:
    times.append(epoch.to_iso())
    a_priori_errors.append(ob.get_residual(sat).range)
    fit_errors.append(ob.get_residual(bls.current_estimate).range)
    

# Add the data to a plotly object
fig = go.Figure()
fig.add_trace(
    go.Scatter(
        x=times,
        y=a_priori_errors,
        mode="markers",
        name="A Priori Errors",
        marker=dict(color="blue"),
    )
)
fig.add_trace(
    go.Scatter(
        x=times,
        y=fit_errors,
        mode="markers",
        name="Fit Errors",
        marker=dict(color="red"),
    )
)
fig.update_layout(
    title="A Priori and Fit Errors",
    xaxis_title="Epoch (UTC)",
    yaxis_title="Error (km)",
    legend=dict(x=0, y=1),
    xaxis_tickformat="%Y-%m-%d %H:%M:%S",
)

fig.show()
```

{!examples/od_example_plot.html!}
