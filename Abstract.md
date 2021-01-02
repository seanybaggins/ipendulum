# Theory of design 
## Dynamic modeling and design of basic pole-placement controller
Begin with the Lagrangian.
$$
L=T-V=K_e -P_e
\tag{1}
$$
Determine the kinetic energies of the cart and pendulum pop, which is treated as a point mass at the end of a mass-less rod. The pendulum bob's position can be treated as such:
$$
p_x=x+lsin\theta \\
p_y=lcos\theta
\tag{2}
$$
This has corresponding velocities:
$$
v_px=\dot x+l\dot\theta cos\theta \\
v_py=-l\dot\theta sin\theta
\tag{3}
$$
Their squared magnitude:
$$
v_p^2=(\dot x+l\dot\theta cos\theta)^2+(-l\dot\theta sin\theta)^2 \\
= \dot x^2 + 2 \dot xl\dot\theta cos\theta+l^2\dot\theta^2cos^2\theta+l^2 \dot\theta^2sin^2\theta \\
=\dot x^2+2\dot xl\dot\theta cos\theta +l^2\dot\theta^2
\tag{4}
$$
Using this for the pendulum's kinetic energy
$$
KE_p=\frac{1}{2}m\dot (\dot x^2+2\dot xl\dot\theta cos\theta +l^2\dot\theta^2) \\
KE_c=\frac{1}{2}M\dot x^2
$$
Only the pendulum contributes to the potential energy of the system, and is defined as:
$$
PE_p=mgl cos\theta
$$
Returning to the Lagrangian:
$$
L = \frac{1}{2}M\dot x^2+\frac{1}{2}m\dot (\dot x^2+2\dot xl\dot\theta cos\theta +l^2\dot\theta^2)-mglcos\theta \\
=\frac{1}{2}(M+m)\dot x^2+m\dot xl\dot\theta cos\theta + \frac{1}{2}ml^2\dot\theta^2-mglcos\theta
\tag{5}
$$
From here we determine the Euler-Lagrange equations of motion (EOM), which are of the form:
$$
\frac{d}{dt}\left\lgroup \frac{\partial L}{\partial \dot x}\right\rgroup-\frac{\partial L}{\partial x}=f \\
\frac{d}{dt}\left\lgroup \frac{\partial L}{\partial \dot \theta}\right\rgroup-\frac{\partial L}{\partial \theta}=0
$$
For the four partial derivative terms, we have:
$$
\frac{\partial L}{\partial \dot x}=(M+m)\dot x+ml\dot\theta cos\theta \\
\frac{\partial L}{\partial \dot \theta}=ml\dot x cos\theta+ml^2\dot\theta \\
\frac{\partial L}{\partial x}=0 \\
\frac{\partial L}{\partial \theta}=-mlsin\theta\dot x\dot\theta+mglsin\theta
$$
And their time derivatives:
$$
\frac{d}{dt}\left\lgroup \frac{\partial L}{\partial \dot x}\right\rgroup=(M+m)\ddot x+ml\ddot\theta cos\theta-ml\dot\theta^2 sin\theta \\
\frac{d}{dt}\left\lgroup \frac{\partial L}{\partial \dot \theta}\right\rgroup=ml^2\ddot\theta+ml\ddot x cos\theta-ml\dot x \dot\theta sin\theta
$$
Substitute everything in, and at last we have a model for the system in the form of two nonlinear second-order ODEs:
$$
(M+m)\ddot x+ml\ddot\theta cos\theta-ml\dot\theta^2 sin\theta=f \\
ml^2\ddot\theta+ml\ddot xcos\theta - mglsin\theta=0
\tag{6}
$$
To make the transition to state-space, first we linearize the above using small-angle approximations:
$$
cos\theta \approxeq1 \\ sin\theta\approxeq\theta \\ \dot\theta^2\approxeq0
$$
Which give the linearized dynamic model:
$$
(M+m)\ddot x + ml\ddot\theta=f \\
m\ddot x +ml\ddot\theta - mg\theta=0
\tag{7}
$$
We use substitutions to solve for each second-order term:
$$
\ddot x=\frac{-mg}{M}\theta + \frac{1}{M}f \\
\ddot\theta=\frac{M+m}{Ml}g\theta - \frac{-1}{Ml}f
$$
With these linearly separable terms, we can represent the system in canonical state-space form:
$$
\dot \bold x=\bold A \bold x+\bold Bu
$$
Where the state vector is defined as:
$$
\bold x = [x, \dot x,\theta, \dot\theta]^T
$$
The system and input matrices follow:
$$
\bold A= \left[
\begin{matrix}
0&1&0&0 \\
0&0&\frac{-mg}{M}&0 \\
0&0&0&1 \\
0&0&\frac{M+m}{Ml}g&0 \\
\end{matrix}
\right] 
\hspace{1cm}
\bold B=\left[
\begin{matrix}
0 \\
\frac{1}{M} \\
0 \\
\frac{-1}{Ml} \\
\end{matrix}
\right]
\tag{8}
$$
Here we introduce the feeback gain matrix **K** and apply it to our input, which we now modify to include an added setpoint. This produces a corresponding closed-loop system:
$$
\bold K=\begin{matrix}[k1&k2&k3&k4]\end{matrix} \\
\bold u=-\bold K \bold y+\bold r\\
\dot \bold x=(\bold A-\bold B \bold K) \bold x+\bold B \bold r \\
\bold y=\bold C\bold x+\bold D \bold u
$$
This closed-loop system has a characteristic equation:
$$
|s\bold I-(\bold A-\bold B \bold K)|=0 \\
s^4+(a_4+k_4)s^3+(a_3+k_3)s^2+(a_2+k_2)s+(a_1+k_1)=0
\tag{9}
$$
To find our gain values we must choose some performance criteria for the transient response, namely the maximum settling time and percent overshoot. This produces a 2nd-order characteristic polynomial to which we add two more terms to make the overall 'desired' polynomial a 4th-order. The added terms simply place then two remaining roots (poles) at 5x the distance of the other two on the real axis. We can then set this equal to our actual characteristic and solve for our gains by matching like-terms.
$$
T_s=\frac{4}{\zeta\omega_n} \\
\zeta=\frac{-\ln(\frac{\%OS}{100})}{\sqrt{\pi^2+\ln^2(\frac{\%OS}{100})}} 
\\
(s^2+2\zeta\omega_ns+\omega_n^2)(s+5\zeta\omega_n)(s+5\zeta\omega_n)=s^4+(a_4+k_4)s^3+(a_3+k_3)s^2+(a_2+k_2)s+(a_1+k_1)
$$
This manual pole-placement can be improved and the system optimally controlled by instead designing a Linear Quadratic Regulator (LQR). This method minimizes a cost function J:
$$
J=\int\limits_0^\infin [\bold x^T \bold Q \bold x+\bold u^T \bold R \bold u]dt
$$
Here **Q** and **R** are symmetric and diagonal and act as weightings for the state and input, respectively. For this application we will assume a negligible cost of control (i.e. assume a wall-powered and sufficiently sized motor). We also treat angular movement of the pendulum with higher relative cost penalty than linear movement of the cart. Our weighting terms could then look similar to the following:
$$
\bold Q= \left[
\begin{matrix}
1&0&0&0 \\
0&1&0&0 \\
0&0&10&0 \\
0&0&0&100 \\
\end{matrix}
\right]
\hspace{1cm}
\bold R=[0.01]
\tag{10}
$$
With weights chosen, gains are determined by solving the algebraic Ricatti equation. With gains chosen for state variable feedback and applying them to the difference of the current state from a desired setpoint, the input signal becomes:
$$
u = -\bold K(\bold x - \bold x_d)
\tag{11}
$$