#import "@preview/cetz:0.3.4": canvas, draw
#import "@preview/cetz-plot:0.1.1": plot

#set page(width: auto, height: auto, margin: 1cm)

#let style = (stroke: black, fill: rgb(0, 0, 200, 75))

#let data = ((0,0), (1,1), (2,4), (3,9))

#set text(font: "New Computer Modern Math", size: 10pt)

#canvas({
  import draw: *

  // Set-up axis and legend styles
  set-style(axes: (stroke: .5pt, tick: (stroke: .5pt)),
            legend: (stroke: none, orientation: ttb, item: (spacing: .3), scale: 80%))

  plot.plot(size: (12, 8),
    x-tick-step: 1,
    y-tick-step: 0.5,
    y-min: -2.5,
    y-max: 10,
    legend: "default",
    {
      plot.add(data, label: $f_1(x)$, style: (stroke: blue, fill: none))
    })
})