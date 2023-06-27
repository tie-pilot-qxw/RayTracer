# Timeline

## Checkpoints and Code **Review**

### Deadline 0: 2023-06-21 23:59

- **Canvas**: 提交 GitHub Repo 链接
- **Checkpoint**: GitHub Actions 成功完成工作流程的 commit（具体表现为有一个绿色的勾并成功输出一张JPG/PNG图片）

### Deadline 1: 2023-06-25 23:59

- **Checkpoint**: book 1 至少以下每条内容写完后需分别 commit
  - A blue-to-white gradient depending on ray Y coordinate

  - A simple red sphere

  - A sphere colored according to its normal vectors

  - An Abstraction for Hittable Objects

  - Resulting render of normals-colored sphere with ground

  - Antialiasing

  - A Simple Diffuse Material

  - Gamma Correction

  - True Lambertian Reflection

  - Hemispherical scattering

  - A Scene with Metal Spheres, Shiny metal, Fuzzed metal

  - Glass(Image 13 - 16)

  - Final Scene

- **口头问答(06-27上午)**：Rust 基础语法特性掌握（范围不超出前 7 章与第 10.2 节）

- **Code Review(06-27上午)**: book 1 相关实现细节

### Deadline 2: 2023-07-2 23:59

- **Checkpoint**: book 2 至少以下每条内容写完后需分别 commit
  
  清晰度要求（width, height, samples per pixel）要求大于等于书上的清晰度。
  
  - Image1：Bouncing spheres。
  
  - BVH（完成Chapter3后）：要求渲染有明显提速
  
    > 我们要求完成BVH前后对某个场景的渲染速度有一个对比。
    >
    > 这一部分你可以拿 **bouncing spheres** 也可以拿 **Book1 Final Scene** 进行实验。观察在渲染同一场景且清晰度相同的情况下，完成BVH后的渲染速度是否有明显提高，如果有，说明你的BVH完成正确。你可以在commit message中注明。
  
  - Image2：Spheres on checkered ground
  
  - Image 7：Hashed random texture
  
  - Image 10：Perlin texture, higher frequency
  
  - Image 13：Perlin noise, marbled texture
  
  - Image 15：Earth-mapped sphere
  
  - Image 17：Scene with rectangle and sphere light sources
  
  - Image 18：Empty Cornell box
  
  - Image 20：Standard Cornell box scene
  
  - Book2 Final scene
  
  > 由于我们只看commit记录的时间，如果清晰度要求下的场景需要渲染很久，你可以先在本地用低清晰度渲染并确保输出图像至少是正确的之后，再调整清晰度为标准并commit到远端仓库用 Github Actions 提供的资源来渲染。并没有必要在本地跑完代码再进行commit。
  
- **Code Review(暂定07-04上午)**: 多线程和 book 2 相关实现细节

