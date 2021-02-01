"""Class providing an abstract graph repository."""
from typing import List, Dict, Set
import os
import compress_json
import datetime
from downloaders import BaseDownloader
from ensmallen_graph import EnsmallenGraph


class GraphRepository:
    def __init__(self):
        """Create new Graph Repository object."""
        self._downloader = BaseDownloader(
            process_number=1,
            verbose=2,
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
        raise NotImplementedError(
            "The method build_stored_graph_name must be implemented in child classes."
        )

    def build_graph_report_path(self, graph_name: str) -> str:
        """Return path where graph report is stored.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to build path for.

        Returns
        -----------------------
        String with path.
        """
        return os.path.join(
            os.path.dirname(os.path.abspath(__file__)),
            self.name,
            "reports",
            "{}.json.gz".format(
                self.build_stored_graph_name(graph_name)
            )
        )

    def load_graph_report(self, graph_name: str) -> Dict:
        """Return dictionary with metadata.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to build path for.

        Returns
        -----------------------
        Metadata relative to the given graph.
        """
        return compress_json.load(
            self.build_graph_report_path(graph_name)
        )

    def store_graph_report(
        self,
        graph_name: str,
        graph_report: str,
        arguments: Dict
    ):
        """Save given graph data into database.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to build path for.
        """
        compress_json.dump(
            {
                "graph_name": graph_name,
                "graph_report": graph_report,
                "datetime": str(datetime.datetime.now()),
                "arguments": arguments,
            },
            self.build_graph_report_path(graph_name)
        )

    def is_graph_cached(self, graph_name: str) -> bool:
        """Return boolean representing if graph is cached.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to check if it is cached.

        Returns
        -----------------------
        Wether the cache if cached.
        """
        return os.path.exists(self.build_graph_report_path(graph_name))

    @property
    def name(self) -> str:
        """Return name of the repository."""
        return self.__class__.__name__

    @property
    def corrupted_graphs_path(self):
        """Return path to corrupted graphs json."""
        return "corrupted_graphs/{}.json.gz".format(self.name)

    def _load_corrupted_graphs(self) -> Set[str]:
        """Return set of known corrupted graphs."""
        if os.path.exists(self.corrupted_graphs_path):
            return compress_json.local_load(self.corrupted_graphs_path)
        return set()

    def _dump_corrupted_graphs(self, corrupted_graphs: Set[str]):
        """Return set of known corrupted graphs."""
        compress_json.local_dump(corrupted_graphs, self.corrupted_graphs_path)

    def add_corrupted_graph(self, graph_name: str):
        """Add given graph to corrupted graphs set.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to add to corrupted set.
        """
        self._dump_corrupted_graphs(
            self._load_corrupted_graphs().add(graph_name)
        )

    def is_graph_corrupted(self, graph_name: str) -> bool:
        """Return boolean representing if graph is known to be corrupted.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to check if it is corrupted.

        Returns
        -----------------------
        Wether the graph is known to be corrupted.
        """
        return graph_name in self._load_corrupted_graphs()

    def get_graph_list(self) -> List[str]:
        """Return list of graph names."""
        raise NotImplementedError(
            "The method get_graph_list must be implemented in child classes."
        )

    def get_uncached_graph_list(self) -> List[str]:
        """Return graphs to be parsed."""
        return [
            graph_name
            for graph_name in self.get_graph_list()
            if not (
                self.is_graph_cached(graph_name) or
                self.is_graph_corrupted(graph_name)
            )
        ]

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
        raise NotImplementedError(
            "The method get_graph_urls must be implemented in child classes."
        )

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
        raise NotImplementedError(
            "The method get_graph_paths must be implemented in child classes."
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
        return dict(
            name=graph_name,
            edge_path=edge_path,
            node_path=node_path,
            directed=False
        )

    def download(self, graph_name: str):
        """Return url for the given graph.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrieve.
        """
        urls = self.get_graph_urls(graph_name)
        return self._downloader.download(
            urls=urls,
            paths=self.get_graph_paths(graph_name, urls)
        )

    def retrieve_all(self):
        """Return all the graph from the repository."""
        for graph_name in self.get_uncached_graph_list():
            download_report = self.download(graph_name)
            if len(download_report) == 1:
                edge_path = download_report.iloc[0].extraction_path
            else:
                edge_path = download_report.iloc[0].extraction_path
                node_path = download_report.iloc[1].extraction_path
            arguments = self.build_graph_parameters(
                graph_name,
                edge_path=edge_path,
                node_path=node_path,
            )
            graph = EnsmallenGraph.from_unsorted_csv(**arguments)
            self.store_graph_report(
                graph_name,
                graph_report=str(graph),
                arguments=arguments
            )