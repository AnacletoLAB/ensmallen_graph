from time import time
from humanize import naturaldelta
from ensmallen_graph import EnsmallenGraph
import compress_json
import json


start = time()
graph = EnsmallenGraph.from_csv(
    "pos_train_edges.tsv",
    "pos_train_nodes.tsv"
)
completed_graph = time() - start
start_walk = time()
graph.walk(
    iterations=10,
    length=80,
    min_length=0,
    return_weight=1,
    explore_weight=1,
    change_node_type_weight=1,
    change_edge_type_weight=1
)
delta = time() - start
total_walk_time = time() - start_walk

response = {
    "required_time": delta,
    "human_time": naturaldelta(delta),
    "building_graph_required_time": completed_graph,
    "building_graph_required_human_time": naturaldelta(completed_graph),
    "random_walk_time": total_walk_time,
    "random_walk_human_time": naturaldelta(total_walk_time)
}

print(json.dumps(response, indent=4))

compress_json.dump(response, "time_required.json", json_kwargs={"indent": 4})