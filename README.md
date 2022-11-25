# Mindustry逻辑强化(MRH)

随着时代发展,传统逻辑已经无法满足开发者们对开发的需要

由此,MRH应运而生

## 什么是MRH?

MRH是一个"编译器",能够将MRH代码编译为Mindustry逻辑代码

## MRH相比传统逻辑有什么优势?

传统逻辑类似汇编,只有很基础的一些方法

很多数据结构,使用传统逻辑难以实现

MRH能够把mhr具有高级特性的MRH代码编译为传统逻辑代码

## MRH增加了哪些特性?

### 数组

开发者可以设置一个定长数组,用来存放任意类型的数据
#### MRH代码
```
// 定义ax为长度为4的数组
let ax = arr 4
```
#### 逻辑代码
```
op mul arrry_ax_jumpline array_ax_index 3
op add @counter @counter arrry_ax_ju
set array_ax_e3 array_ax_this
set array_ax_this array_ax_e0
set @counter ...
set array_ax_e0 array_ax_this
set array_ax_this array_ax_e1
set @counter ...
set array_ax_e1 array_ax_this
set array_ax_this array_ax_e2
set @counter ...
set array_ax_e2 array_ax_this
set array_ax_this array_ax_e3
set @counter ...

```
### 读取简化
对Sensor命令的简化

支持简写,加快开发速度

#### 对照表

MRH         传统逻辑

ti          totalItem

fi          firstItem

tl          totalLiquids

tp          totalPower

ic          itemCapacity

lc          LiquidsCapacity

pc          powerCapacity

pns         powerNetStored

pnc         powerNetCapacity

pni         powerNetIn

pno         powerNetOut

am          ammo

ac          ammoCapacity

hp          health

hm          maxHealth

ht          heat

ef          effciency

pg          progress

ts          timescale

rn          rotation

x           x

y           y

sx          shootX

sy          shootY

si          shooting

sz          size

gg          dead

rg          range

bi          boosting

mx          mineX

my          mineY

mi          mining

sp          speed

tm          team

ty          type

fl          flag

cd          controlled

cr          controller

nm          name

pc          payloadCount

pt          payloadType

ea          enabled

cf          config

cl          color

#### MRH代码
```
let udead = gg in @unit
// in是一个运算符!
```
#### 传统逻辑代码
```
TODO
```
### 复杂表达式

对原版op的强化

支持复杂的表达式

#### MRH代码
```
let x = 114 * (514 + 1919810)
```
#### 传统逻辑代码
```
op add op_t0 514 1919810
op mul x 114 op_t0
```
sin,cos,tan...
rand.floor...
等函数,会变成伪函数

举例:
#### MRH代码
```
let r = rand(114514)
```
#### 传统逻辑代码
```
op rand x 114514 0
```

### 处理器协同

原版处理器算力低下

而多处理器协同,并不容易

MRH让这个困难简单一丢丢

#### MRH代码
```
pg{
    write(cell1,0,1)
    write(cell1,1,0)
    write(cell1,2,1)
    write(cell1,3,0)
    write(cell1,4,1)
    write(cell1,5,0)
    write(cell1,6,1)
    write(cell1,7,0)
    write(cell1,8,1)
}
pg{
    for i 0 3
        for i 0 3
            let data = read(cell1,i * 3 + j)
            let ox = i * 30
            let oy = j * 30
            if data
                draw_rect(ox,oy,30,30)
    draw_flush(display1)
}
```
#### 传统逻辑代码
处理器1
```
write 1 cell1 0
write 0 cell1 1
write 1 cell1 2
write 0 cell1 3
write 1 cell1 4
write 0 cell1 5
write 1 cell1 6
write 0 cell1 7
write 1 cell1 8

```
处理器2
```
set i 0
jump 15 equal i 3
set j 0
jump 13 equal j 3
op mul op_t0 i 3
op add op_t1 i j
read data cell1 op_t1
op mul ox i 30
op mul oy j 30
jump 11 notEqual data 1
draw rect ox oy 30 30 0 0
op add j j 1
set @counter 3
op add i i 1
set @counter 1
drawflush display1

```