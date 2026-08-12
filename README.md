# Novagrad

Backpropogation engine with a focus on individual node control. TUI included

Could you change the rendering of the model run screen to highlight the selected metric on the side panel instead of as tabs?
I.e, I want the user to be able to cycle through the available metrics using the up and down keys, the selected metric on the side panel will be highlighted and a graph will be shown on the main panel if it is timeseries data, otherwise leave the main panel blank.
Order the metrics alphabetically when displayed.
If there are none, show some sort of default message like "No metrics available".
Create another panel underneath the side panel to show logs in realtime (I will add the logic for that later).
In the layout area where the tabs used to be, please put in controls like pause/start/save for controlling the model run.
Here are the files that may be helpful:
