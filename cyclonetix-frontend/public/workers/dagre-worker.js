self.importScripts('https://cdnjs.cloudflare.com/ajax/libs/cytoscape/3.23.0/cytoscape.min.js');
self.importScripts('https://cdnjs.cloudflare.com/ajax/libs/dagre/0.8.5/dagre.min.js');
self.importScripts('https://cdn.jsdelivr.net/npm/cytoscape-dagre@2.5.0/cytoscape-dagre.min.js');

// Register layout with Cytoscape
self.cytoscape.use(self.dagre);

self.addEventListener('message', function(e) {
  const { nodes, edges, layoutOptions } = e.data;

  try {
    // Create a headless instance of cytoscape for layout calculation only
    const cy = self.cytoscape({
      headless: true,
      elements: {
        nodes: nodes,
        edges: edges
      }
    });

    // Report progress periodically
    let progressReported = 0;
    const interval = setInterval(() => {
      progressReported += 0.1;
      if (progressReported < 0.9) {
        self.postMessage({ type: 'progress', progress: progressReported });
      } else {
        clearInterval(interval);
      }
    }, 200);

    // Run the layout
    const layout = cy.layout(layoutOptions);

    layout.one('layoutready', function() {
      clearInterval(interval);
      self.postMessage({ type: 'progress', progress: 0.9 });
    });

    layout.run();

    // Extract the position data
    const positionData = {};
    cy.nodes().forEach(node => {
      positionData[node.id()] = {
        x: node.position('x'),
        y: node.position('y')
      };
    });

    // Send the calculated positions back to the main thread
    self.postMessage({ type: 'layoutComplete', positions: positionData });

    // Clean up
    cy.destroy();
  } catch (error) {
    self.postMessage({ type: 'error', error: error.toString() });
  }
});
