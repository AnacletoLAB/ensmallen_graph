"""Sub-module handling the retrieval and building of graphs from STRING."""
from typing import List, Dict
import os
import pandas as pd
from .graph_repository import GraphRepository


class StringGraphRepository(GraphRepository):

    def __init__(self):
        """Create new String Graph Repository object."""
        super().__init__()
        self._base_url = "https://string-db.org/cgi/download?species_text={}"
        self._organisms = pd.read_csv(
            "https://stringdb-static.org/download/species.v11.0.txt",
            sep="\t"
        )

    def build_stored_graph_name(self, partial_graph_name: str) -> str:
        """Return built graph name.

        Parameters
        -----------------------
        partial_graph_name: str,
            Partial graph name to be built.

        Returns
        -----------------------
        Complete name of the graph.
        """
        return "".join([
            term.capitalize()
            for term in partial_graph_name.split(" ")
        ])

    def get_graph_urls(self, graph_name: str) -> List[str]:
        """Return url for the given graph.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrievel URLs for.

        Returns
        -----------------------
        The urls list from where to download the graph data.
        """
        return self._base_url.format(graph_name.replace(" ", "+"))

    def get_graph_paths(self, graph_name: str, urls: List[str]) -> List[str]:
        """Return url for the given graph.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrievel URLs for.
        urls: List[str],
            Urls from where to download the graphs.

        Returns
        -----------------------
        The paths where to store the downloaded graphs.
        """
        return os.path.join(
            self.name,
            "{}.csv.gz".format(
                graph_name.lower().replace(" ", "_")
            )
        )

    def build_graph_parameters(
        self,
        graph_name: str,
        edge_path: str,
        node_path: str = None,
    ) -> Dict:
        """Return dictionary with kwargs to load graph.

        Parameters
        ---------------------
        graph_name: str,
            Name of the graph to load.
        edge_path: str,
            Path from where to load the edge list.
        node_path: str = None,
            Optionally, path from where to load the nodes.

        Returns
        -----------------------
        Dictionary to build the graph object.
        """
        return {
            **super().build_graph_parameters(
                graph_name,
                edge_path,
                node_path
            ),
            "edge_separator": " ",
            "sources_column": "protein1",
            "destinations_column": "protein2",
            "weights_column": "combined_score",
        }

    def get_graph_list(self) -> List[str]:
        """Return list of graph names."""
        return list(set(self._organisms.STRING_name_compact))
