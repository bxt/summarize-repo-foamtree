var foamtree = null;

function addParent(group, parent) {
  group.parent = parent;
  if (group.groups) {
    group.groups.forEach(function (g) {
      addParent(g, group);
    });
  }
}

function setOpenAll(group, value) {
  group.open = value;
  if (group.groups) {
    group.groups.forEach(function (g) {
      setOpenAll(g, value);
    });
  }
}

var sortedPersonsColors = [
  "#333333FF",
  "#FFFF00FF",
  "#DDBB00FF",
  "#FF3399FF",
  "#FF0000FF",
  "#FF9900FF",
  "#0000ffFF",
  "#AA000FF",
  "#990000FF",
  "#3333FFFF",
  "#000099FF",
  "#444444FF",
  "#444444FF",
  "#444444FF",
  "#444444FF",
  "#444444FF",
  "#444444FF",
  "#440099FF",
];
var sortedPersons = [];

function summarizePersons(group) {
  if (group.person) {
    group.persons = {[group.person]: group.weight};
  }
  if (group.groups) {
    group.persons = {};
    group.groups.forEach(function (g) {
      summarizePersons(g);
      for (person in g.persons) {
        group.persons[person] ||= 0;
        group.persons[person] += g.persons[person];
      }
    });
  }
  if (!group.parent) {
    sortedPersons = Object.entries(group.persons);
    sortedPersons.sort((a, b) => b[1] - a[1]);
    console.log(sortedPersons);
  }
}

window.addEventListener("load", function () {
  foamtree = new CarrotSearchFoamTree({
    id: "visualization",
    dataObject,
    attributionTheme: "dark",
    // Alternate layout:
    // layout: "squarified",
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
    // descriptionGroupSize: 0.1,
    // Less border:
    groupBorderWidth: 2,
    groupInsetWidth: 4,
    // No border and children full visible:
    groupBorderWidth: 0,
    groupInsetWidth: 1,
    parentFillOpacity: 0,
    maxGroupLevelsDrawn: 8,
    // Store references to parent groups
    onModelChanging: function (group) {
      addParent(group, undefined);
      setOpenAll(group, true);
      summarizePersons(group);
    },
    onGroupClick: function (e) {
      e.preventDefault();
      var group = e.secondary ? e.bottommostOpenGroup : e.topmostClosedGroup;
      var toZoom;
      if (group) {
        // this.open({ groups: group, open: !e.secondary });
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
        // this.open({ groups: group, open: false });
        toZoom = group.parent;
      } else {
        toZoom = this.get("dataObject");
      }
      this.zoom(toZoom);
    },

    groupColorDecorator: function (options, properties, variables) {
      let colorIndex = sortedPersons.findIndex(([p,v]) => p === properties.group.person);
      variables.groupColor = sortedPersonsColors[colorIndex] || "#00112200";
      if (properties.group.person !== undefined) {
        variables.labelColor = "#00000000";
      } else {
        variables.labelColor = "#ffffff33";
      }
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
