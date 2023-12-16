var foamtree = null;

window.addEventListener("load", function () {
  foamtree = new CarrotSearchFoamTree({
    id: "visualization",
    dataObject,
    attributionTheme: "dark",
    // Alternate layout:
    layout: "squarified",
    // "Performance"
    rolloutDuration: 0,
    pullbackDuration: 0,
    groupMinDiameter: 0,
    maxGroupLevelsDrawn: 2,
    maxGroupLabelLevelsDrawn: 2,
    wireframeLabelDrawing: "always",
    // Cool labels:
    // stacking: "flattened",
    // descriptionGroupType: "floating",
    // descriptionGroupSize: 0.4,
    // Less border:
    groupBorderWidth: 2,
    groupInsetWidth: 4,
    // Store references to parent groups
    onModelChanging: function addParent(group, parent) {
      group.parent = parent;
      if (group.groups) {
        group.groups.forEach(function(g) {
          addParent(g, group);
        });
      }
    },
    onGroupClick: function (e) {
      e.preventDefault();
      var group = e.secondary ? e.bottommostOpenGroup : e.topmostClosedGroup;
      var toZoom;
      if (group) {
        this.open({ groups: group, open: !e.secondary });
        toZoom = e.secondary ? group.parent : group;
      } else {
        toZoom = this.get("dataObject");
      }
      this.zoom(toZoom);
    },
    onGroupDoubleClick: function (e) {
      e.preventDefault();
      var group = e.bottommostOpenGroup;
      var toZoom;
      if (group) {
        this.open({ groups: group, open: false });
        toZoom = group.parent;
      } else {
        toZoom = this.get("dataObject");
      }
      this.zoom(toZoom);
    },
  });
});

window.addEventListener(
  "resize",
  (function () {
    var timeout;
    return function () {
      window.clearTimeout(timeout);
      if (foamtree) {
        timeout = window.setTimeout(foamtree.resize, 300);
      }
    };
  })()
);
