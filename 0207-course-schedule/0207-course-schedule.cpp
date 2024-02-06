class Solution {
      void dfs(int src, vector<vector<int>> &adj, vector<bool> &vis, vector<bool> &isAnc, bool &ans)
      {
          if(ans)
              return;
          
          vis[src] = true;
          isAnc[src] = true;
          
          for(int v: adj[src])
          {
              if(ans)
                  return;
              
              else if(isAnc[v])
              {
                  ans = true;
                  cout << v << ' ' << (src == v) << '\n';
                  return;
              }
              
              else if(!vis[v])
                  dfs(v, adj, vis, isAnc, ans);
          }
          
          isAnc[src] = false;
      }
public:
    bool canFinish(int numCourses, vector<vector<int>>& prerequisites) {
            int n = numCourses;
        vector<vector<int>> adj(n);
            for(int i = 0; i < (int)prerequisites.size(); i++){
                adj[prerequisites[i][0]].push_back(prerequisites[i][1]);
            }
        vector<bool> vis(n), isAnc(n);
        bool ans = false;
        for(int i = 0 ;i < n; i++)
            if(!vis[i])
                dfs(i, adj, vis, isAnc, ans);
        return !ans;
            
            
            
            
    }
};