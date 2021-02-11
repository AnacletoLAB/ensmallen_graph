"""
This file offers the methods to automatically retrieve the graph Ruminococcaceae bacterium AB4001.

The graph is automatically retrieved from the STRING repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-03 22:43:35.157453

The undirected graph Ruminococcaceae bacterium AB4001 has 2667 nodes and
156094 weighted edges, of which none are self-loops. The graph is dense
as it has a density of 0.04391 and has 5 connected components, where the
component with most nodes has 2657 nodes and the component with the least
nodes has 2 nodes. The graph median node degree is 91, the mean node degree
is 117.06, and the node degree mode is 6. The top 5 most central nodes
are 1410638.JHXJ01000040_gene502 (degree 985), 1410638.JHXJ01000011_gene682
(degree 961), 1410638.JHXJ01000007_gene1046 (degree 782), 1410638.JHXJ01000004_gene2021
(degree 750) and 1410638.JHXJ01000002_gene1386 (degree 718).


References
---------------------
Please cite the following if you use the data:

@article{szklarczyk2019string,
    title={STRING v11: protein--protein association networks with increased coverage, supporting functional discovery in genome-wide experimental datasets},
    author={Szklarczyk, Damian and Gable, Annika L and Lyon, David and Junge, Alexander and Wyder, Stefan and Huerta-Cepas, Jaime and Simonovic, Milan and Doncheva, Nadezhda T and Morris, John H and Bork, Peer and others},
    journal={Nucleic acids research},
    volume={47},
    number={D1},
    pages={D607--D613},
    year={2019},
    publisher={Oxford University Press}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.string import RuminococcaceaeBacteriumAb4001

    # Then load the graph
    graph = RuminococcaceaeBacteriumAb4001()

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


def RuminococcaceaeBacteriumAb4001(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/string"
) -> EnsmallenGraph:
    """Return new instance of the Ruminococcaceae bacterium AB4001 graph.

    The graph is automatically retrieved from the STRING repository. 

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
    Instace of Ruminococcaceae bacterium AB4001 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-03 22:43:35.157453
	
	The undirected graph Ruminococcaceae bacterium AB4001 has 2667 nodes and
	156094 weighted edges, of which none are self-loops. The graph is dense
	as it has a density of 0.04391 and has 5 connected components, where the
	component with most nodes has 2657 nodes and the component with the least
	nodes has 2 nodes. The graph median node degree is 91, the mean node degree
	is 117.06, and the node degree mode is 6. The top 5 most central nodes
	are 1410638.JHXJ01000040_gene502 (degree 985), 1410638.JHXJ01000011_gene682
	(degree 961), 1410638.JHXJ01000007_gene1046 (degree 782), 1410638.JHXJ01000004_gene2021
	(degree 750) and 1410638.JHXJ01000002_gene1386 (degree 718).
	

	References
	---------------------
	Please cite the following if you use the data:
	
	@article{szklarczyk2019string,
	    title={STRING v11: protein--protein association networks with increased coverage, supporting functional discovery in genome-wide experimental datasets},
	    author={Szklarczyk, Damian and Gable, Annika L and Lyon, David and Junge, Alexander and Wyder, Stefan and Huerta-Cepas, Jaime and Simonovic, Milan and Doncheva, Nadezhda T and Morris, John H and Bork, Peer and others},
	    journal={Nucleic acids research},
	    volume={47},
	    number={D1},
	    pages={D607--D613},
	    year={2019},
	    publisher={Oxford University Press}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.string import RuminococcaceaeBacteriumAb4001
	
	    # Then load the graph
	    graph = RuminococcaceaeBacteriumAb4001()
	
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
        "RuminococcaceaeBacteriumAb4001",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="string"
    )()