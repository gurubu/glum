#version 440
in vec4 gl_FragCoord;
out vec3 col;
//uniform float shp1x;
//uniform float shp1y;
//uniform float shp1r;
//uniform float shp2x;
//uniform float shp2y;
//uniform float shp2r;
//uniform int sdfo;
uniform block{
  vec2 posx;
  vec2 posy;
  vec2 radius;
};
float smin(float a,float b,float k){
  float res = exp2(-k*a)+exp2(-k*b);
  return -log2(res)/k;
}
float sphere(vec2 p,float r){return length(p)-r;}
float scene(vec2 p){return 
  smin(
     sphere(vec2(p.x+posx[0],p.y+posy[1]),radius[0]),
     sphere(vec2(p.x+posx[1],p.y+posy[1]),radius[1]),
     80.0);}
//float scene(vec2 p){
//  if (sdfo == 1){
//    return smin(sphere(vec2(p.x+shp1x,p.y+shp1y),shp1r),sphere(vec2(p.x+shp2x,p.y+shp2y),shp2r),5.0);
//  }
//  if (sdfo == 2){
//    return max(-sphere(vec2(p.x+shp1x,p.y+shp1y),shp1r),sphere(vec2(p.x+shp2x,p.y+shp2y),shp2r));
//  }
//  if (sdfo == 3){
//    return max(sphere(vec2(p.x+shp1x,p.y+shp1y),shp1r),sphere(vec2(p.x,p.y+0.2),0.2));
//  }
//}
//void raymarching(){
//  float depth = 0.0;
//}
vec2 rez = vec2(640.0,480.0);
void main(){
  vec2 q = (gl_FragCoord.xy+1)/2.0;
	vec2 p = (2.0*gl_FragCoord.xy-rez.xy)/rez.y;
  vec2 st = gl_FragCoord.xy/rez.xy; 
  vec2 sy = (st+1.0)/2.0;
  //float r = 0.0;
  //if(scene(p)<0){
  //  r = 0.0;
  //}
  //else{
  //  r = 1.0;
  //}
  float r = 0.0;
  if(scene(p)>0.0){
   r = 1.0; 
  }
  float g = 0.0;
  float b = 0.0;
  col=vec3(r,g,b);
}
