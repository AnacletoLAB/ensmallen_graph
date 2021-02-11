"""
This file offers the methods to automatically retrieve the graph SW-10000-6-0d3-L5.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-04 08:38:18.091920

The undirected graph SW-10000-6-0d3-L5 has 10000 nodes with 5 different
node types: 1 (nodes number 2000), 5 (nodes number 2000), 4 (nodes number
2000), 3 (nodes number 2000) and 2 (nodes number 2000) and 30000 unweighted
edges, of which none are self-loops. The graph is quite sparse as it has
a density of 0.00060 and is connected, as it has a single component. The
graph median node degree is 6, the mean node degree is 6.00, and the node
degree mode is 6. The top 5 most central nodes are 2895 (degree 12), 9933
(degree 11), 9631 (degree 11), 9476 (degree 11) and 7235 (degree 11).


References
---------------------
Please cite the following if you use the data:

@inproceedings{nr,
    title = {The Network Data Repository with Interactive Graph Analytics and Visualization},
    author={Ryan A. Rossi and Nesreen K. Ahmed},
    booktitle = {AAAI},
    url={http://networkrepository.com},
    year={2015}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import Sw1000060d3L5

    # Then load the graph
    graph = Sw1000060d3L5()

    # Finally, you can do anything with it, for instance, compute its report:
    print(graph)

    # If you need to run a link prediction task with validation,
    # you can split the graph using a connected holdout as follows:
    train_graph, validation_graph = graph.connected_holdout(
        # You can use an 80/20 split the holdout, for example.
        train_size=0.8,
        # The random state is used to reproduce the holdout.
        random_state=42,
        # Wether to show a loading bar.
        verbose=True
    )

    # Remember that, if you need, you can enable the memory-time trade-offs:
    train_graph.enable(
        vector_sources=True,
        vector_destinations=True,
        vector_outbounds=True
    )

    # Consider using the methods made available in the Embiggen package
    # to run graph embedding or link prediction tasks.
"""
from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen_graph import EnsmallenGraph  # pylint: disable=import-error


def Sw1000060d3L5(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the SW-10000-6-0d3-L5 graph.

    The graph is automatically retrieved from the NetworkRepository repository. 

    Parameters
    -------------------
    directed: bool = False,
        Wether to load the graph as directed or undirected.
        By default false.
    verbose: int = 2,
        Wether to show loading bars during the retrieval and building
        of the graph.
    cache_path: str = "graphs",
        Where to store the downloaded graphs.

    Returns
    -----------------------
    Instace of SW-10000-6-0d3-L5 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-04 08:38:18.091920
	
	The undirected graph SW-10000-6-0d3-L5 has 10000 nodes with 5 different
	node types: 1 (nodes number 2000), 5 (nodes number 2000), 4 (nodes number
	2000), 3 (nodes number 2000) and 2 (nodes number 2000) and 30000 unweighted
	edges, of which none are self-loops. The graph is quite sparse as it has
	a density of 0.00060 and is connected, as it has a single component. The
	graph median node degree is 6, the mean node degree is 6.00, and the node
	degree mode is 6. The top 5 most central nodes are 2895 (degree 12), 9933
	(degree 11), 9631 (degree 11), 9476 (degree 11) and 7235 (degree 11).
	

	References
	---------------------
	Please cite the following if you use the data:
	
	@inproceedings{nr,
	    title = {The Network Data Repository with Interactive Graph Analytics and Visualization},
	    author={Ryan A. Rossi and Nesreen K. Ahmed},
	    booktitle = {AAAI},
	    url={http://networkrepository.com},
	    year={2015}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import Sw1000060d3L5
	
	    # Then load the graph
	    graph = Sw1000060d3L5()
	
	    # Finally, you can do anything with it, for instance, compute its report:
	    print(graph)
	
	    # If you need to run a link prediction task with validation,
	    # you can split the graph using a connected holdout as follows:
	    train_graph, validation_graph = graph.connected_holdout(
	        # You can use an 80/20 split the holdout, for example.
	        train_size=0.8,
	        # The random state is used to reproduce the holdout.
	        random_state=42,
	        # Wether to show a loading bar.
	        verbose=True
	    )
	
	    # Remember that, if you need, you can enable the memory-time trade-offs:
	    train_graph.enable(
	        vector_sources=True,
	        vector_destinations=True,
	        vector_outbounds=True
	    )
	
	    # Consider using the methods made available in the Embiggen package
	    # to run graph embedding or link prediction tasks.
    """
    return AutomaticallyRetrievedGraph(
        "Sw1000060d3L5",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()