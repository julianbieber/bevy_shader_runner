#version 450
layout(location = 0) in vec2 uv;

layout(location = 0) out vec4 fragColor;

layout(set = 2, binding = 0) uniform float iTime;
layout(set = 2, binding = 1) uniform vec2 iResolution;
layout(set = 2, binding = 2) uniform vec4 slider1;
layout(set = 2, binding = 3) uniform vec4 slider2;
layout(set = 2, binding = 4) uniform vec4 slider3;
layout(set = 2, binding = 5) uniform vec4 slider4;
layout(set = 2, binding = 6) uniform vec4 slider5;

mat3 rot(float a, float b, float c) {
    return mat3(1.0, 0.0, 0.0,
        0.0, cos(a), -sin(a),
        0.0, sin(a), cos(a)) *
        mat3(cos(b), -sin(b), 0.0,
            sin(b), cos(b), 0.0,
            0.0, 0.0, 1.0) *
        mat3(cos(c), 0.0, -sin(c),
            0.0, 1.0, 0.0,
            sin(c), 0.0, cos(c));
}

float hash(vec3 p) {
    p = fract(p * 0.1031);
    p += sin(dot(p, p.zyx + 31.32)*23.32);
    return fract((p.x + p.y) * p.z);
}

float cnoise(vec3 p) {
    vec3 ip = floor(p);
    vec3 fp = fract(p);

    float a = hash(ip + vec3(0, 0, 0));
    float b = hash(ip + vec3(1, 0, 0));
    float c = hash(ip + vec3(0, 1, 0));
    float d = hash(ip + vec3(1, 1, 0));
    float e = hash(ip + vec3(0, 0, 1));
    float f = hash(ip + vec3(1, 0, 1));
    float g = hash(ip + vec3(0, 1, 1));
    float h = hash(ip + vec3(1, 1, 1));

    return mix(
        mix(mix(a, b, fp.x), mix(c, d, fp.x), fp.y),
        mix(mix(e, f, fp.x), mix(g, h, fp.x), fp.y),
        fp.z
    );
}

float fbm(vec3 x, float H) {
    float G = exp2(-H);
    float f = 1.0;
    float a = 1.0;
    float t = 0.0;
    for( int i=0; i<2; i++ )
    {
        t += a*cnoise(f*x);
        f *= 2.0;
        a *= G;
    }
    return t;
}

float sdBox( vec3 p, vec3 b )
{
    float r = 0.1;
    vec3 q = abs(p) - b + r;
    return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0) - r;
}

vec3 map(vec3 p) {
    vec3 op = p;


    float an = 6.283185 / (15.0);

    float a = atan(p.x, p.z);
    float s = round(a / an);
    float os = s;
    float ra = s * an;

    p = p * rot(0.0, 0.0, ra);
    p.xz = p.xz - 30.0*round(p.xz/30.0);
    if (p.y < 3.0) {
        p.y = p.y - 1*round(p.y/1.0);
    }

    an = 6.283185 / (12.0);
    a = atan(p.x, p.z);
    s = round(a / an);
    ra = s * an;

    p = p * rot(0.0, 0.0, ra);

    float dBrick = sdBox(p - vec3(0.1, 0.1, 3.0), vec3(0.5, 0.2, 0.2)) - fbm(p*20.0*rot(0.3, .2, 0.1), 0.5)*0.01;
    float ground = op.y + 5.0-fbm(op*3.0, 0.7)*0.2;

    float d = min(dBrick, ground);
    // d = dBrick;

    float v = 0.0;
    if (d==dBrick) {
        v = 1.0;
    } else if (d == ground) {
        v = 5.0;
    }

    return vec3(d,v, dBrick);
}
vec3 map2( in vec3 p )
{
   float d = sdBox(p,vec3(4.0)*fbm(p, 0.8));
   vec3 res = vec3( d, 1.0, 0.0, 0.0 );

   float s = 1.0;
   for( int m=0; m<3; m++ )
   {
      p = p * rot(0.4, 0.2, 0.3);
      p.y *= 1.2;
      p.z *= 0.7;
      vec3 a = mod( p*s, 2.0 )-1.0;
      s *= 3.0;
      vec3 r = abs(1.0 - 3.0*abs(a));

      float da = max(r.x,r.y);
      float db = max(r.y,r.z);
      float dc = max(r.z,r.x);
      float c = (min(da,min(db,dc))-1.0)/s;

      if( c>d )
      {
          d = c;
          res = vec3( d, 0.2*da*db*dc, (1.0+float(m))/4.0, 0.0 );
       }
   }

   return res;
}

vec3 normal(vec3 p) {
    float ep = 0.001;
    vec2 h = vec2(ep, 0.0);
    return normalize(vec3(
            map(p + h.xyy).x - map(p - h.xyy).x,
            map(p + h.yxy).x - map(p - h.yxy).x,
            map(p + h.yyx).x - map(p - h.yyx).x
        ));
}

vec3 c1(float s) {
    vec3 a = vec3(0.0, 0.0, 0.0);
    vec3 b = vec3(0.2, 0.3, 0.2);
    vec3 c = vec3(0.1, 0.1, 0.2);
    vec3 d = vec3(0.0, 0.0, 0.0);

    return a + b * cos(6.283185 * s + d);
}
vec3 c2(float s) {
    vec3 a = vec3(0.2, 0.2, 0.2);
    vec3 b = vec3(0.3, 0.1, 0.1);
    vec3 c = vec3(0.2, 0.1, 0.2);
    vec3 d = vec3(0.0, 0.0, 0.0);

    return a + b * cos(6.283185 * s + d);
}
vec3 c3(float s) {
    vec3 a = vec3(0.0, 0.0, 0.0);
    vec3 b = vec3(0.1, 0.1, 0.3);
    vec3 c = vec3(0.3, 0.2, 0.1);
    vec3 d = vec3(0.0, 0.0, 0.0);

    return a + b * cos(6.283185 * s + d);
}

vec3 march(vec3 ro, vec3 rd) {
    float t = 0.0;
    vec3 p = vec3(0.0);
    float min_d = 1000000.0;
    vec3 d = vec3(0.0);

    for (int i = 0; i < 100; ++i) {
        p = ro + rd * t;

        d = map(p);

        t += d.x*0.8;
        min_d = min(min_d, d.z);
        if (d.x < 0.001) {
            vec3 n = normal(p);
            float i = dot(n, normalize(vec3(0.0, 1.0, 1.0))* rot(iTime, 0.0, 0.0));

            if (d.y == 1.0) {
                return vec3(1.0, 0.0, 0.0) * i;
            }
            if (d.y == 2.0) {
                return vec3(0.0, 1.0, 0.0) * i;
            }
            if (d.y == 3.0) {
                return vec3(0.0, 0.0, 1.0) * i;
            }
            if (d.y == 4.0) {
                return vec3(0.0, 1.0, 1.0) * i;
            }
            if (d.y == 5.0) {
                vec3 a = c3(abs(min_d+t*0.0001)+2.1);
                vec3 b = c2(abs(min_d+t*0.0001)+2.1);
                vec3 c = c1(abs(min_d+t*0.0001)+2.1);
                return mix(a, mix(b,c,sin(p.x)), sin(p.z)) * i;
            }
        }
        if (t > 100.0) {
            break;
        }
    }
    if (min_d < 1.0) {
        float s = 1.0 - min_d;
        return vec3(0.0);
    } else {
        return vec3(0.0);
    }
}

void main() {
    vec3 rd = vec3(uv, -1.0);
    rd -= 0.5;
    rd *= 2.0;
    rd.y *= -1.0;
    rd.x *= iResolution.x / iResolution.y;
    // rd.x *= -1.0;
    rd.z = -1.0;
    rd = normalize(rd);
    // rd.y -= 1.0;
    rd = rd * rot(slider2.x*10.0, slider2.y*10.0, slider2.z*10.0);

    vec3 ro = vec3(0.0, 7.0 + 10.0 * (slider2.w-0.5), 10.0+(slider1.w-0.5)*10.0);
    fragColor = vec4(march(ro, rd), 1.0);
    // fragColor = vec4(abs(rd), 0.0);
}
