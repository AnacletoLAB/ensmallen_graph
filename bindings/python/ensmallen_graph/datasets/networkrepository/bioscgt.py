"""
This file offers the methods to automatically retrieve the graph bio-SC-GT.

The graph is automatically retrieved from the NetworkRepository repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-03 22:30:17.144635

The undirected graph bio-SC-GT has 1716 nodes and 33987 weighted edges,
of which none are self-loops. The graph is dense as it has a density of
0.02310 and has 4 connected components, where the component with most nodes
has 1708 nodes and the component with the least nodes has 2 nodes. The
graph median node degree is 21, the mean node degree is 39.61 and the node
degree mode is 1. The top 5 most central nodes are 35 (degree 549), 29
(degree 413), 116 (degree 242), 521 (degree 237) and 519 (degree 237).


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

@article{cho2014wormnet,
        title={WormNet v3: a network-assisted hypothesis-generating server for Caenorhabditis elegans},
        author={Cho, Ara and Shin, Junha and Hwang, Sohyun and Kim, Chanyoung and Shim, Hongseok and Kim, Hyojin and Kim, Hanhae and Lee, Insuk},
        journal={Nucleic acids research},
        volume={42},
        number={W1},
        pages={W76--W82},
        year={2014},
        publisher={Oxford University Press}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import BioScGt

    # Then load the graph
    graph = BioScGt()

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


def BioScGt(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the bio-SC-GT graph.

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
    Instace of bio-SC-GT graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-03 22:30:17.144635
	
	The undirected graph bio-SC-GT has 1716 nodes and 33987 weighted edges,
	of which none are self-loops. The graph is dense as it has a density of
	0.02310 and has 4 connected components, where the component with most nodes
	has 1708 nodes and the component with the least nodes has 2 nodes. The
	graph median node degree is 21, the mean node degree is 39.61 and the node
	degree mode is 1. The top 5 most central nodes are 35 (degree 549), 29
	(degree 413), 116 (degree 242), 521 (degree 237) and 519 (degree 237).
	

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
	
	@article{cho2014wormnet,
	        title={WormNet v3: a network-assisted hypothesis-generating server for Caenorhabditis elegans},
	        author={Cho, Ara and Shin, Junha and Hwang, Sohyun and Kim, Chanyoung and Shim, Hongseok and Kim, Hyojin and Kim, Hanhae and Lee, Insuk},
	        journal={Nucleic acids research},
	        volume={42},
	        number={W1},
	        pages={W76--W82},
	        year={2014},
	        publisher={Oxford University Press}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import BioScGt
	
	    # Then load the graph
	    graph = BioScGt()
	
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
        "BioScGt",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="networkrepository"
    )()