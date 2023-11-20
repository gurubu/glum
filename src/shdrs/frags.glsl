#version 440
in vec4 gl_FragCoord;
out vec3 col;
float smin(float a,float b,float k){
  float res = exp2(-k*a)+exp2(-k*b);
  return -log2(res)/k;
}
float sphere(vec2 p,float r){return length(p)-r;}
float scene(vec2 p){
  return smin(sphere(vec2(p.x,p.y-0.2),0.2),sphere(vec2(p.x,p.y+0.2),0.2),35.0);
}
void raymarching(){
  float depth = 0.0;
}
vec2 rez = vec2(320.0,240.0);
void main(){
	vec2 p = (2.0*gl_FragCoord.xy-rez.xy)/rez.y;
  vec2 st = gl_FragCoord.xy/rez.xy; 
  vec2 sy = (st+1.0)/2.0;
  float r = 0.0;
  if(scene(p)<0){
    r = 0.0;
  }
  else{
    r = 1.0;
  }
  float g = 0.0;
  float b = 1.0;
  col=vec3(r,g,b);
}
