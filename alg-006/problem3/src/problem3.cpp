#include <iostream>
#include <fstream>
#include <vector>
#include <sstream>
#include <utility>
#include <set>
#include <random>
#include <algorithm>

using namespace std;

inline
size_t min(size_t a, size_t b) {
  return (a < b ? a : b);
}
inline
size_t max(size_t a, size_t b) {
  return (a > b ? a : b);
}

typedef struct {
  size_t id;
  vector<size_t> adjacent;
} node;

istream& operator>>(istream& is, node &n) {
  string line;

  getline(is, line);

  stringstream ss(line);
  ss >> n.id;
  n.adjacent.clear();
  while(!ss.eof()) {
    size_t i;
    ss >> i;
    n.adjacent.push_back(i);
  }

  return is;
}

void collapse_nodes(size_t from_id, size_t to_id, vector<pair<size_t,size_t> > &edges) {
  for(vector<pair<size_t,size_t> >::iterator iter = edges.begin(); iter != edges.end(); ++iter) {
    if((*iter).first == from_id) {
      (*iter).first = to_id;
    }
    if((*iter).second == from_id) {
      (*iter).second = to_id;
    }
  }
}

void remove_loops(vector<pair<size_t,size_t> > edges) {
}

int main(int argc, char **argv) {
  if(argc != 2) {
    cerr << "Usage: " << argv[0] << " <filename>" << endl;
  }
  string filename(argv[1]);
  std::ifstream ifs;
  ifs.open(filename.c_str(), ifstream::in);

  vector<node> nodes;
  
  while(ifs.good()) {
    node n;
    ifs >> n;
    nodes.push_back(n);
  }

  set<pair<size_t,size_t> > edge_set;

  for(size_t i = 0; i < nodes.size(); ++i) {
    for(size_t j = 0; j < nodes[i].adjacent.size(); ++j) {
      pair<size_t,size_t> edge(min(nodes[i].id, nodes[i].adjacent[j]), max(nodes[i].id, nodes[i].adjacent[j]));
      edge_set.insert(edge);
    }
  }

  vector<pair<size_t,size_t> > edges(edge_set.begin(), edge_set.end());

  cerr << "Edges: " << edges.size() << endl;

  set<size_t> unique_nodes;
  for(size_t i = 0; i < edges.size(); ++i) {
    unique_nodes.insert(edges[i].first);
    unique_nodes.insert(edges[i].second);
  }
  cerr << "Nodes: " << unique_nodes.size() << endl;
  collapse_nodes(200, 1, edges);
  unique_nodes.clear();
  for(size_t i = 0; i < edges.size(); ++i) {
    unique_nodes.insert(edges[i].first);
    unique_nodes.insert(edges[i].second);
  }
  cerr << "Nodes: " << unique_nodes.size() << endl;
}


