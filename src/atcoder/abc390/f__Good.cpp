#include<iostream>
#include<cassert>
using namespace std;
int pidx[3<<17],len[3<<17];
int N;
int main()
{
  ios::sync_with_stdio(false);
  cin.tie(nullptr);
  cin>>N;
  for(int a=0;a<=N+1;a++)pidx[a]=-1,len[a]=0;
  long ans=0;
  long sum=0;
  for(int i=0;i<N;i++)
  {
    int a;cin>>a;
    sum-=len[a];
    if(pidx[a]<=pidx[a-1])
    {
      len[a]=i-pidx[a-1];
    }
    else
    {
      len[a]+=i-pidx[a];
    }
    sum+=len[a];
    if(pidx[a+1]!=-1)
    {
      sum-=len[a+1];
      len[a+1]=0;
    }
    pidx[a]=i;
    ans+=sum;
  }
  cout<<ans<<"\n";
}
