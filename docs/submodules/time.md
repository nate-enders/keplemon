# keplemon.time

## Basic Usage

```python
from keplemon.time import Epoch, TimeSpan
from keplemon.enums import TimeSystem

epoch = Epoch.from_iso("2025-04-02T04:42:42.123", TimeSystem.UTC)
span = TimeSpan.from_minutes(17.42)

new_epoch = epoch + span
time_components = new_epoch.to_time_components()

print("new_epoch.to_iso()")
print(f"Year: {time_components.year}")
print(f"Month: {time_components.month}")
print(f"Day: {time_components.day}")
print(f"Hour: {time_components.hour}")
print(f"Minute: {time_components.minute}")
print(f"Second: {time_components.second}")
```

```text
2025-04-02T05:00:7.323
Year: 2025
Month: 4
Day: 2
Hour: 5
Minute: 0
Second: 7.323000056
```

::: keplemon.time
