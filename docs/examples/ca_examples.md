# Close Approach Examples

## Comparing a Constellation to a Satellite

```python title="ca_many_vs_one.py"

from keplemon.bodies import Satellite, Constellation
from keplemon.elements import TLE
from keplemon.catalogs import TLECatalog
from keplemon.time import TimeSpan

fengyun_deb_tles = TLECatalog.from_tle_file("2025-05-08-fengyun-deb.3le")
fengyun_deb = Constellation.from_tle_catalog(fengyun_deb_tles)

iss_tle = TLE.from_lines(
    "1 25544U 98067A   25128.47843163  .00008677  00000+0  16314-3 0  9999",
    "2 25544  51.6344 143.7626 0002306  89.9507 270.1746 15.49411867509000",
)
iss = Satellite.from_tle(iss_tle)

ca_start = iss_tle.epoch
ca_end = ca_start + TimeSpan.from_days(1)
ca_distance = 100.0

report = fengyun_deb.get_ca_report_vs_one(iss, ca_start, ca_end, ca_distance)
print(f"Number of conjunctions: {len(report.close_approaches)}")
for ca in report.close_approaches:
    print(f"Conjunction between {ca.primary_id} and {ca.secondary_id}:")
    print(f"  Time:     {ca.epoch.to_iso()}")
    print(f"  Distance: {ca.distance:.2f} km\n")
```

```text title="terminal"
Number of conjunctions: 3
Conjunction between 25544 and 32363:
  Time:     2025-05-09T04:57:05.990
  Distance: 75.39 km

Conjunction between 25544 and 30420:
  Time:     2025-05-08T21:40:31.284
  Distance: 67.68 km

Conjunction between 25544 and 33721:
  Time:     2025-05-09T01:10:18.468
  Distance: 98.49 km
```

## Comparing a Constellation to Itself

```python title="ca_many_vs_many.py"
from keplemon.bodies import Constellation
from keplemon.catalogs import TLECatalog
from keplemon.time import TimeSpan, Epoch
from keplemon.enums import TimeSystem

geo_tles = TLECatalog.from_tle_file("2025-05-08-active-geo.3le")
active_geo = Constellation.from_tle_catalog(geo_tles)

ca_start = Epoch.from_iso("2025-05-08T00:00:00.000", TimeSystem.UTC)
ca_end = ca_start + TimeSpan.from_days(1)
ca_distance = 10.0

report = active_geo.get_ca_report_vs_many(ca_start, ca_end, ca_distance)
print(f"Number of conjunctions: {len(report.close_approaches)}")
for ca in report.close_approaches:
    print(f"Conjunction between {ca.primary_id} and {ca.secondary_id}:")
    print(f"  Time:     {ca.epoch.to_iso()}")
    print(f"  Distance: {ca.distance:.2f} km\n")
```

```text title="terminal"
Number of conjunctions: 6
Conjunction between 39163 and 44334:
  Time:     2025-05-08T08:50:56.788
  Distance: 9.52 km

Conjunction between 40733 and 49055:
  Time:     2025-05-08T17:05:26.766
  Distance: 7.85 km

Conjunction between 36581 and 39617:
  Time:     2025-05-08T13:37:05.232
  Distance: 6.81 km

Conjunction between 29045 and 35755:
  Time:     2025-05-08T18:24:49.406
  Distance: 9.56 km

Conjunction between 33274 and 41729:
  Time:     2025-05-08T05:05:11.202
  Distance: 8.50 km

Conjunction between 28945 and 37264:
  Time:     2025-05-08T04:59:25.288
  Distance: 7.95 km
```
